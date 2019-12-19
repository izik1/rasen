use crate::params::mem::{Displacement, SIB};
use crate::params::{
    mem::{Memory, ModRM},
    GeneralRegister, Immediate, WWidth, WidthAtLeast16,
};
use crate::{Assembler, WritableImmediate, REXB, REXW, REXR};
use std::io;

include!(concat!(env!("OUT_DIR"), "/fns.rs"));

// todo: maybe redirect to `op_reg_imm` if `Width == 8` instead of WidthAtLeast16 trait bound?
//  above is not doable without const_generics, because `WidthAtLeast16` != W8,
//  and `Width::IS_W8` doesn't prove (to the compiler anyway) that `Width == W8`

impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {
    fn op_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        opcode: u8,
        rm_bits: u8,
        imm: i8,
    ) -> io::Result<()> {
        let initial_rex = if reg.needs_rex() {
            REXB
        } else {
            0b0000_0000
        };

        self.op_rm_imm::<Width>(
            (ModRM::new(0b11, rm_bits, reg.value() % 8), None, None),
            WritableImmediate::W8(imm as u8),
            // this is unused, since Width >= 16, but we have to put _something_ there.
            opcode,
            opcode,
            initial_rex,
        )
    }

    fn op_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        opcode: u8,
        rm_bits: u8,
        imm: i8,
    ) -> io::Result<()> {
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let (mod_rm, sib, displacement) = mem.encoded();

        self.op_rm_imm::<Width>(
            (mod_rm.with_op(rm_bits), sib, displacement),
            WritableImmediate::W8(imm as u8),
            // this is unused, since Width >= 16, but we have to put _something_ there.
            opcode,
            opcode,
            mem.rex_byte(),
        )
    }

    fn op_rm_imm<Width: WWidth>(
        &mut self,
        mod_bytes: (ModRM, Option<SIB>, Option<Displacement>),
        imm: WritableImmediate,
        op8: u8,
        op: u8,
        initial_rex: u8,
    ) -> io::Result<()> {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let mut rex_byte = initial_rex;

        if Width::HAS_REXW {
            rex_byte |= REXW;
        }

        if rex_byte != 0x00 {
            self.write_byte(rex_byte)?;
        }

        let opcode: u8 = if Width::IS_W8 { op8 } else { op };

        self.write_byte(opcode)?;

        self.write_mod_rm(mod_bytes.0)?;

        if let Some(sib) = mod_bytes.1 {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = mod_bytes.2 {
            self.write_displacement(displacement)?;
        }

        self.write_immediate(imm)
    }

    fn op_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
        op8: u8,
        op: u8,
        rm_bits: u8,
    ) -> io::Result<()> {
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let (mod_rm, sib, displacement) = mem.encoded();

        self.op_rm_imm::<Width>(
            (mod_rm.with_op(rm_bits), sib, displacement),
            imm.as_writable(),
            op8,
            op,
            mem.rex_byte(),
        )
    }

    fn op_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
        op8: u8,
        op: u8,
        rm_bits: u8,
    ) -> io::Result<()> {
        let mut initial_rex = if reg.needs_rex() { 0b0100_0001 } else { 0 };

        // SPL, BPL, SIL, DIL are the registers that this matters for.
        if Width::IS_W8 && reg.value() >= 4 {
            // SPL, BPL, SIL, DIL
            initial_rex |= 0b0100_0000;
        }

        self.op_rm_imm::<Width>(
            (ModRM::new(0b11, rm_bits, reg.value() % 8), None, None),
            imm.as_writable(),
            op8,
            op,
            initial_rex,
        )
    }

    fn op_zax_imm<Width: WWidth>(
        &mut self,
        imm: impl Immediate<Width>,
        op8: u8,
        op: u8,
    ) -> io::Result<()> {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        if Width::HAS_REXW {
            self.write_byte(REXW)?;
        }

        let opcode: u8 = if Width::IS_W8 { op8 } else { op };

        self.write_byte(opcode)?;

        self.write_immediate(imm.as_writable())
    }

    fn op_rm_mr<Width: WWidth, R, M>(&mut self, reg: R, mem: M, op8: u8, op: u8) -> io::Result<()> where R: GeneralRegister<Width>, M: Memory<M> {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let mut rex = mem.rex_byte();

        if reg.needs_rex() {
            rex |= REXR;
        }

        // SPL, BPL, SIL, DIL are the registers that this matters for.
        if Width::IS_W8 && reg.value() >= 4 {
            // SPL, BPL, SIL, DIL
            rex |= 0b0100_0000;
        }

        if Width::HAS_REXW {
            rex |= REXW;
        }

        if rex != 0 {
            self.write_byte(rex)?;
        }

        let opcode: u8 = if Width::IS_W8 { op8 } else { op };

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_reg(reg.value() % 8))?;

        if let Some(sib) = sib {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = displacement {
            self.write_displacement(displacement)?;
        }

        Ok(())
    }

    // xor_hi8_imm(Hi8Bit, u8)
    // xor_mem_hi8(Mem<W8>, Hi8Bit)
    // xor_hi8_mem(Hi8Bit, Mem<W8>)
}
