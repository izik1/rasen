use crate::params::{GeneralRegister, Immediate, WWidth, WidthAtLeast16};
use crate::Assembler;
use std::io;

include!(concat!(env!("OUT_DIR"), "/fns.rs"));

impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {
    pub fn xor_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let mut rex_byte = 0_u8;
        if reg.needs_rexb() {
            rex_byte |= 0b0100_0001;
        }

        if Width::HAS_REXW {
            rex_byte |= 0b0100_1000;
        }

        // SPL, BPL, SIL, DIL are the registers that this matters for.
        if Width::IS_W8 && reg.value() >= 4 {
            // SPL, BPL, SIL, DIL
            rex_byte |= 0b0100_0000;
        }

        let opcode: u8 = if Width::IS_W8 { 0x80 } else { 0x81 };

        if rex_byte != 0 {
            self.write_byte(rex_byte)?;
        }

        self.write_byte(opcode)?;

        // register = 0b1100_0000
        // opcode = 0b110
        //
        let mod_rm = 0b1111_0000 | (reg.value() % 8);

        self.write_byte(mod_rm)?;

        self.write_immediate(imm.as_writable())
    }

    // xor_hi8_imm<Width>(Hi8Bit, u8)
    // xor_mem_imm<Width>(Mem<Width>, Immediate<Width>)

    pub fn xor_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        // todo: maybe redirect to `xor_reg_imm` if `Width == 8` instead of this trait bound?
        // ^ above is not doable without const_generics, because `WidthAtLeast16` != W8,
        // and `Width::IS_W8` doesn't prove (to the compiler anyway) that `Width == W8`

        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let mut rex_byte = 0;
        if reg.needs_rexb() {
            rex_byte |= 0b0100_0001;
        }

        if Width::HAS_REXW {
            rex_byte |= 0b0100_1000;
        }

        if rex_byte != 0 {
            self.write_byte(rex_byte)?;
        }

        // opcode
        self.write_byte(0x83)?;

        // register = 0b1100_0000
        // opcode = 0b110
        //
        let mod_rm = 0b1111_0000 | (reg.value() % 8);
        self.write_byte(mod_rm)?;

        self.write_byte(imm as u8)
    }

    // xor_mem_sximm8<Width>(Mem<Width>, i8)
    // xor_mem_reg<Width>(Mem<Width>, Register<Width>)
    // xor_mem_hi8(Mem<W8>, Hi8Bit)
    // xor_reg_mem<Width>(Register<Width>, Mem<Width>)
    // xor_hi8_mem(Hi8Bit, Mem<W8>)
}
