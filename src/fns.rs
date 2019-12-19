use crate::params::{GeneralRegister, Immediate, WWidth, WidthAtLeast16, Mem};
use crate::Assembler;
use std::io;

include!(concat!(env!("OUT_DIR"), "/fns.rs"));

// todo: maybe redirect to `op_reg_imm` if `Width == 8` instead of WidthAtLeast16 trait bound?
//  above is not doable without const_generics, because `WidthAtLeast16` != W8,
//  and `Width::IS_W8` doesn't prove (to the compiler anyway) that `Width == W8`


impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {
    // xor_hi8_imm(Hi8Bit, u8)
    // xor_mem_imm<Width>(Mem<Width>, Immediate<Width>)
    pub fn xor_mem_imm<Width: WWidth>(&mut self, mem: Mem, imm: impl Immediate<Width>) -> io::Result<()> {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let mut rex_byte = mem.rex_byte();

        if Width::HAS_REXW {
            rex_byte |= 0b0100_1000;
        }

        let opcode: u8 = if Width::IS_W8 {
            0x80
        } else {
            0x81
        };

        if rex_byte != 0x00 {
            self.write_byte(rex_byte)?;
        }

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_op(6))?;

        if let Some(sib) = sib {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = displacement {
            self.write_displacement(displacement)?;
        }

        self.write_immediate(imm.as_writable())
    }

    // xor_mem_sximm8<Width>(Mem<Width>, i8)
    // xor_mem_reg<Width>(Mem<Width>, Register<Width>)
    // xor_mem_hi8(Mem<W8>, Hi8Bit)
    // xor_reg_mem<Width>(Register<Width>, Mem<Width>)
    // xor_hi8_mem(Hi8Bit, Mem<W8>)
}
