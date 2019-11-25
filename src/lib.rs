use std::io::{self, Seek, SeekFrom, Write};

// This is for an x86 assembler for now, TODO: move into a specific module for x86?

pub struct Assembler<'a, T>
where
    T: Write + Seek,
{
    writer: &'a mut T,
    start_offset: u64,
    current_offset: u64,
}

impl<'a, T> Assembler<'a, T>
where
    T: Write + Seek,
{
    pub fn new(writer: &'a mut T) -> io::Result<Self> {
        let start_offset = writer.seek(SeekFrom::Current(0))?;
        Ok(Self {
            start_offset,
            current_offset: start_offset,
            writer,
        })
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)?;
        self.current_offset += buf.len() as u64;
        Ok(())
    }

    #[inline(always)]
    pub fn start_offset(&self) -> u64 {
        self.start_offset
    }

    #[inline(always)]
    pub fn current_offset(&self) -> u64 {
        self.current_offset
    }

    #[inline(always)]
    pub fn write_byte(&mut self, byte: u8) -> io::Result<()> {
        self.write_all(&byte.to_le_bytes())
    }

    #[inline(always)]
    pub fn write_word(&mut self, word: u16) -> io::Result<()> {
        self.write_all(&word.to_le_bytes())
    }

    #[inline(always)]
    pub fn write_dword(&mut self, dword: u32) -> io::Result<()> {
        self.write_all(&dword.to_le_bytes())
    }

    #[inline(always)]
    pub fn write_qword(&mut self, qword: u64) -> io::Result<()> {
        self.write_all(&qword.to_le_bytes())
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
    fn errors_when_out_of_space() {
        let mut writer = create_writer(1);
        let mut assembler = Assembler::new(&mut writer).unwrap();
        assert_eq!(
            assembler.write_word(0xfefa).unwrap_err().kind(),
            io::ErrorKind::WriteZero
        );
    }

    #[test]
    fn writes_byte() -> io::Result<()> {
        let mut writer = create_writer(1);
        let mut assembler = Assembler::new(&mut writer)?;
        assembler.write_byte(0xfe)?;

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 1);

        assert_eq!(&*writer.into_inner(), &[0xfe]);

        Ok(())
    }

    #[test]
    fn writes_word() -> io::Result<()> {
        let mut writer = create_writer(2);
        let mut assembler = Assembler::new(&mut writer)?;
        assembler.write_word(0xfefa)?;

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 2);

        assert_eq!(&*writer.into_inner(), &[0xfa, 0xfe]);

        Ok(())
    }

    #[test]
    fn writes_dword() -> io::Result<()> {
        let mut writer = create_writer(4);
        let mut assembler = Assembler::new(&mut writer)?;
        assembler.write_dword(0xfefa97ab)?;

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 4);

        assert_eq!(&*writer.into_inner(), &[0xab, 0x97, 0xfa, 0xfe]);
        Ok(())
    }

    #[test]
    fn writes_qword() -> io::Result<()> {
        let mut writer = create_writer(8);
        let mut assembler = Assembler::new(&mut writer)?;
        assembler.write_qword(0xfefa97ab23518719)?;

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 8);

        assert_eq!(
            &*writer.into_inner(),
            &[0x19, 0x87, 0x51, 0x23, 0xab, 0x97, 0xfa, 0xfe]
        );

        Ok(())
    }
}
