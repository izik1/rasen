use std::io::{self, Seek, SeekFrom, Write};

use std::collections::HashMap;

mod emitter;
mod labeler;

// This is for an x86 assembler for now, TODO: move into a specific module for x86?

use emitter::Emitter;
use labeler::Labeler;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Label(usize);

// unresolved todo: should assembler impl Drop?
pub struct Assembler<'a, T>
where
    T: Write + Seek,
{
    emitter: Emitter<'a, T>,
    unresolved_labels: HashMap<u64, Label>,
    labeler: Labeler,
}

impl<'a, T> Assembler<'a, T>
where
    T: Write + Seek,
{
    pub fn new(writer: &'a mut T) -> io::Result<Self> {
        Ok(Self {
            emitter: Emitter::new(writer)?,
            unresolved_labels: HashMap::new(),
            labeler: Labeler::new(),
        })
    }

    pub fn finish(mut self) -> io::Result<()> {
        for (use_addr, label) in self.unresolved_labels.drain() {
            match self.labeler.resolve_label(label) {
                Some(label_addr) => {
                    self.emitter
                        .write_qword_seek(SeekFrom::Start(use_addr), label_addr)?;
                }

                None => todo!("What should we do if we failed to resolve a label while finishing?"),
            }
        }

        Ok(())
    }

    /// Creates a label
    pub fn make_label(&mut self) -> Label {
        self.labeler.create_label()
    }

    /// Creates a label and attaches it at the current RIP
    pub fn make_label_attached(&mut self) -> Label {
        self.labeler.create_attached_label(self.current_offset())
    }

    /// Attaches a label at the current RIP
    /// It is a logic error to use a label from a different instance.
    pub fn attach_label(&mut self, label: Label) {
        self.labeler.attach_label(label, self.current_offset())
    }

    /// Writes the value 64-bit value of a label relative to the current PC out to the stream.
    ///
    /// If the label hasn't been attached yet, it will store it in an internal buffer to resolve it on finish.
    ///
    /// It is a logic error to use a label from a different instance.
    /// # panics
    /// If the label isn't contained in this assembler.
    pub fn write_label(&mut self, label: Label) -> io::Result<()> {
        // if the label hasn't been resolved yet we should make sure that we can resolve it when `finish` is called.
        match self.labeler.resolve_label(label) {
            Some(label_value) => {
                let value = label_value.wrapping_sub(self.current_offset());
                self.write_qword(value)
            }

            None => {
                // todo: should we panic if there's already a label here?
                self.unresolved_labels.insert(self.current_offset(), label);
                // temporarily write UD2 x 4 just in case this somehow gets executed as code.
                // (Even though this is a relative address)
                self.write_qword(0x0f0b0f0b0f0b0f0b)
            }
        }
    }

    #[inline(always)]
    pub fn start_offset(&self) -> u64 {
        self.emitter.start_offset()
    }

    #[inline(always)]
    pub fn current_offset(&self) -> u64 {
        self.emitter.current_offset()
    }

    #[inline(always)]
    pub fn write_byte(&mut self, byte: u8) -> io::Result<()> {
        self.emitter.write_byte(byte)
    }

    #[inline(always)]
    pub fn write_word(&mut self, word: u16) -> io::Result<()> {
        self.emitter.write_word(word)
    }

    #[inline(always)]
    pub fn write_dword(&mut self, dword: u32) -> io::Result<()> {
        self.emitter.write_dword(dword)
    }

    #[inline(always)]
    pub fn write_qword(&mut self, qword: u64) -> io::Result<()> {
        self.emitter.write_qword(qword)
    }
}

#[cfg(test)]
mod test {
    use super::Assembler;
    use std::io::{self, Cursor};

    fn create_writer(size: usize) -> Cursor<Box<[u8]>> {
        let inner = vec![0; size].into_boxed_slice();
        Cursor::new(inner)
    }

    #[test]
    fn writes_label_behind() -> io::Result<()> {
        let mut writer = create_writer(9);
        let mut assembler = Assembler::new(&mut writer)?;
        let label = assembler.make_label_attached();

        assembler.write_byte(0xfe)?;
        assembler.write_label(label)?;

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 9);

        assert_eq!(
            &*writer.into_inner(),
            &[0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );

        Ok(())
    }

    #[test]
    fn writes_label_ahead() -> io::Result<()> {
        let mut writer = create_writer(8);
        let mut assembler = Assembler::new(&mut writer)?;
        let label = assembler.make_label();
        assembler.write_label(label)?;

        assembler.attach_label(label);

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 8);

        assembler.finish()?;

        assert_eq!(
            &*writer.into_inner(),
            &[0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn errors_on_unattached_label() {
        let mut writer = create_writer(8);
        let mut assembler = Assembler::new(&mut writer).unwrap();
        let label = assembler.make_label();
        assembler.write_label(label).unwrap();

        // _don't_ attach the label, this should cause `finish` to error.
        // assembler.attach_label(label);

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 8);

        // this should panic due to the unattached label.
        assembler.finish().unwrap();
    }
}
