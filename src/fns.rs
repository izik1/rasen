use crate::params::mem::{Displacement, SIB};
use crate::params::{
    mem::{Memory, ModRM},
    GeneralRegister, Immediate, WWidth, WidthAtLeast16, WidthAtLeast32, WidthAtMost32, W16, W8, W64,
};
use crate::{Assembler, Vex, WritableImmediate, REXB, REXR, REXW, REXX};
use std::io;

include!(concat!(env!("OUT_DIR"), "/fns.rs"));

// todo: maybe redirect to `op_reg_imm` if `Width == 8` instead of WidthAtLeast16 trait bound?
//  above is not doable without const_generics, because `WidthAtLeast16` != W8,
//  and `Width::IS_W8` doesn't prove (to the compiler anyway) that `Width == W8`

impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {
    fn op_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
        op8: u8,
        op: u8,
        rm_bits: u8,
    ) -> io::Result<()> {
        let reg = reg.into();
        let initial_rex = if reg.needs_rex() { REXB } else { 0b0000_0000 };

        self.op_rm::<Width>(
            (ModRM::new(0b11, rm_bits, reg.writable()), None, None),
            Some(WritableImmediate::W8(imm)),
            // this is unused, since Width >= 16, but we have to put _something_ there.
            op8,
            op,
            initial_rex,
        )
    }

    fn op_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
        op8: u8,
        op: u8,
        rm_bits: u8,
    ) -> io::Result<()> {
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let (mod_rm, sib, displacement) = mem.encoded();

        self.op_rm::<Width>(
            (mod_rm.with_op(rm_bits), sib, displacement),
            Some(WritableImmediate::W8(imm)),
            // this is unused, since Width >= 16, but we have to put _something_ there.
            op8,
            op,
            mem.rex_byte(),
        )
    }

    fn op_reg<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R, op8: u8, op: u8, rm_bits: Option<u8>, mm: Option<u8>) -> io::Result<()> {
        let reg = reg.into();
        let initial_rex = if reg.needs_rex() { REXB } else { 0b0000_0000 };

        if let Some(mm) = mm {
            self.write_byte(mm)?;
        }

        self.op_rm::<Width>(
            (ModRM::new(0b11, rm_bits.unwrap_or(0), reg.writable()), None, None),
            None,
            // this is unused, since Width >= 16, but we have to put _something_ there.
            op8,
            op,
            initial_rex,
        )
    }

    fn op_mem<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        op8: u8,
        op: u8,
        rm_bits: Option<u8>, mm: Option<u8>,
    ) -> io::Result<()> {
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        if let Some(mm) = mm {
            self.write_byte(mm)?;
        }

        let (mod_rm, sib, displacement) = mem.encoded();

        self.op_rm::<Width>(
            (mod_rm.with_op(rm_bits.unwrap_or(0)), sib, displacement),
            None,
            // this is unused, since Width >= 16, but we have to put _something_ there.
            op8,
            op,
            mem.rex_byte(),
        )
    }

    fn op_reg_reg_reg<Width: WWidth, RD, RS1, RS2>(
        &mut self,
        r1: RD,
        r2: RS1,
        r3: RS2,
        mm: u8,
        op: u8,
        pp: u8,
    ) -> io::Result<()>
    where
        RD: GeneralRegister<Width>,
        RS1: GeneralRegister<Width>,
        RS2: GeneralRegister<Width>,
    {
        // RD gets VEX.R
        // RS1 gets VEX.B
        // VEX.X doesn't exist
        // RS2 gets vvvv

        let rd = r1.into();
        let rs1 = r2.into();
        let rs2 = r3.into();

        let r = !rd.needs_rex();
        let b = !rs1.needs_rex();

        let vex = Vex::new((!(rs2 as u8)) & 0xf, pp, mm, r, true, b, Width::IS_W64);

        let mod_rm = ModRM::new(0b11, rd.writable(), rs1.writable());

        self.write_vex(vex)?;
        self.write_byte(op)?;
        self.write_mod_rm(mod_rm)
    }

    fn op_reg_mem_reg<Width: WWidth, R1, M, R2>(
        &mut self,
        r1: R1,
        mem: M,
        r2: R2,
        mm: u8,
        op: u8,
        pp: u8,
    ) -> io::Result<()>
    where
        R1: GeneralRegister<Width>,
        M: Memory<Width>,
        R2: GeneralRegister<Width>,
    {
        // r1 gets VEX.R
        // r2 gets vvvv

        let rd = r1.into();
        let mem = mem.into();
        let rs = r2.into();

        let r = !rd.needs_rex();
        let x = (mem.rex_byte() & REXX) == 0;
        let b = (mem.rex_byte() & REXB) == 0;

        let vex = Vex::new((!(rs as u8)) & 0xf, pp, mm, r, x, b, Width::IS_W64);

        let (mod_rm, sib, displacement) = mem.encoded();
        let mod_rm = mod_rm.with_reg(rd.writable());

        self.write_vex(vex)?;
        self.write_byte(op)?;
        self.write_mod_rm(mod_rm)?;

        if let Some(sib) = sib {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = displacement {
            self.write_displacement(displacement)?;
        }

        Ok(())
    }

    fn op_rm<Width: WWidth>(
        &mut self,
        mod_bytes: (ModRM, Option<SIB>, Option<Displacement>),
        imm: Option<WritableImmediate>,
        op8: u8,
        op: u8,
        initial_rex: u8,
    ) -> io::Result<()> {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let mut rex_byte = initial_rex;

        if Width::IS_W64 {
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

        if let Some(imm) = imm {
            self.write_immediate(imm)?;
        }

        Ok(())
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

        self.op_rm::<Width>(
            (mod_rm.with_op(rm_bits), sib, displacement),
            Some(imm.as_writable()),
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
        let reg = reg.into();

        let mut initial_rex = if reg.needs_rex() { 0b0100_0001 } else { 0 };

        // SPL, BPL, SIL, DIL are the registers that this matters for.
        if Width::IS_W8 && reg.value() >= 4 {
            // SPL, BPL, SIL, DIL
            initial_rex |= 0b0100_0000;
        }

        self.op_rm::<Width>(
            (ModRM::new(0b11, rm_bits, reg.writable()), None, None),
            Some(imm.as_writable()),
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

        if Width::IS_W64 {
            self.write_byte(REXW)?;
        }

        let opcode: u8 = if Width::IS_W8 { op8 } else { op };

        self.write_byte(opcode)?;

        self.write_immediate(imm.as_writable())
    }

    fn op_rm_mr<Width: WWidth, R, M>(
        &mut self,
        reg: R,
        mem: M,
        op8: u8,
        op: u8,
        prefix: Option<u8>,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let reg = reg.into();
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

        if Width::IS_W64 {
            rex |= REXW;
        }

        if rex != 0 {
            self.write_byte(rex)?;
        }

        if let Some(prefix) = prefix {
            self.write_byte(prefix)?;
        }

        let opcode: u8 = if Width::IS_W8 { op8 } else { op };

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_reg(reg.writable()))?;

        if let Some(sib) = sib {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = displacement {
            self.write_displacement(displacement)?;
        }

        Ok(())
    }

    // note: reg1 gets written to reg and reg2 gets written to R/M
    fn op_reg_reg<Width: WWidth, R1, R2>(
        &mut self,
        reg1: R1,
        reg2: R2,
        op8: u8,
        op: u8,
        prefix: Option<u8>,
    ) -> io::Result<()>
    where
        R1: GeneralRegister<Width>,
        R2: GeneralRegister<Width>,
    {
        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let mut rex = 0;

        let reg1 = reg1.into();
        let reg2 = reg2.into();

        if reg1.needs_rex() {
            rex |= REXR;
        }

        if reg2.needs_rex() {
            rex |= REXB;
        }

        // SPL, BPL, SIL, DIL are the registers that this matters for.
        if Width::IS_W8 && (reg1.value() >= 4 || reg2.value() >= 4) {
            // SPL, BPL, SIL, DIL
            rex |= 0b0100_0000;
        }

        if Width::IS_W64 {
            rex |= REXW;
        }

        if rex != 0 {
            self.write_byte(rex)?;
        }

        if let Some(prefix) = prefix {
            self.write_byte(prefix)?;
        }

        let opcode: u8 = if Width::IS_W8 { op8 } else { op };

        self.write_byte(opcode)?;

        let mod_rm = ModRM::new(0b11, reg1.writable(), reg2.writable());

        self.write_mod_rm(mod_rm)?;

        Ok(())
    }

    fn op_no_operands(&mut self, opcode: u8, prefix: Option<u8>) -> io::Result<()> {
        if let Some(prefix) = prefix {
            self.write_byte(prefix)?;
        }

        self.write_byte(opcode)
    }

    pub fn mov_reg_imm64<R: GeneralRegister<W64>>(&mut self, reg: R, imm: u64) -> io::Result<()> {
        let reg = reg.into();

        if reg.needs_rex() {
            self.write_byte(REXW | REXR)?;
        } else {
            self.write_byte(REXW)?;
        }

        let opcode: u8 = 0xb8;

        self.write_byte(opcode)?;

        self.write_immediate(WritableImmediate::W64(imm))?;

        Ok(())
    }

    pub fn movzx_reg_mem8<Width: WidthAtLeast16, R: GeneralRegister<Width>, M: Memory<W8>>(
        &mut self,
        reg: R,
        mem: M,
    ) -> io::Result<()> {
        let op = 0xb6;
        let prefix = Some(0x0f);

        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let reg = reg.into();
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let mut rex = mem.rex_byte();

        if reg.needs_rex() {
            rex |= REXR;
        }

        if Width::IS_W64 {
            rex |= REXW;
        }

        if rex != 0 {
            self.write_byte(rex)?;
        }

        if let Some(prefix) = prefix {
            self.write_byte(prefix)?;
        }

        let opcode: u8 = op;

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_reg(reg.writable()))?;

        if let Some(sib) = sib {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = displacement {
            self.write_displacement(displacement)?;
        }

        Ok(())
    }

    pub fn movzx_reg_mem16<Width: WidthAtLeast32, R: GeneralRegister<Width>, M: Memory<W16>>(
        &mut self,
        reg: R,
        mem: M,
    ) -> io::Result<()> {
        let op = 0xb7;
        let prefix = Some(0x0f);

        let reg = reg.into();
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let mut rex = mem.rex_byte();

        if reg.needs_rex() {
            rex |= REXR;
        }

        if Width::IS_W64 {
            rex |= REXW;
        }

        if rex != 0 {
            self.write_byte(rex)?;
        }

        if let Some(prefix) = prefix {
            self.write_byte(prefix)?;
        }

        let opcode: u8 = op;

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_reg(reg.writable()))?;

        if let Some(sib) = sib {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = displacement {
            self.write_displacement(displacement)?;
        }

        Ok(())
    }

    pub fn movsx_reg_mem8<Width: WidthAtLeast16, R: GeneralRegister<Width>, M: Memory<W8>>(
        &mut self,
        reg: R,
        mem: M,
    ) -> io::Result<()> {
        let op = 0xbe;
        let prefix = Some(0x0f);

        if Width::IS_W16 {
            self.write_byte(0x66)?;
        }

        let reg = reg.into();
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let mut rex = mem.rex_byte();

        if reg.needs_rex() {
            rex |= REXR;
        }

        if Width::IS_W64 {
            rex |= REXW;
        }

        if rex != 0 {
            self.write_byte(rex)?;
        }

        if let Some(prefix) = prefix {
            self.write_byte(prefix)?;
        }

        let opcode: u8 = op;

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_reg(reg.writable()))?;

        if let Some(sib) = sib {
            self.write_sib(sib)?;
        }

        if let Some(displacement) = displacement {
            self.write_displacement(displacement)?;
        }

        Ok(())
    }

    pub fn movsx_reg_mem16<Width: WidthAtLeast32, R: GeneralRegister<Width>, M: Memory<W16>>(
        &mut self,
        reg: R,
        mem: M,
    ) -> io::Result<()> {
        let op = 0xbe;
        let prefix = Some(0x0f);

        let reg = reg.into();
        let mem = mem.into();

        if let Some(prefix) = mem.address_prefix() {
            self.write_byte(prefix)?;
        }

        let mut rex = mem.rex_byte();

        if reg.needs_rex() {
            rex |= REXR;
        }

        if Width::IS_W64 {
            rex |= REXW;
        }

        if rex != 0 {
            self.write_byte(rex)?;
        }

        if let Some(prefix) = prefix {
            self.write_byte(prefix)?;
        }

        let opcode: u8 = op;

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_reg(reg.writable()))?;

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

#[cfg(test)]
mod test {
    use crate::params::{Reg32, Register};
    use crate::Assembler;
    use std::io;
    use std::io::Cursor;

    fn create_writer(size: usize) -> Cursor<Box<[u8]>> {
        let inner = vec![0; size].into_boxed_slice();
        Cursor::new(inner)
    }

    #[test]
    fn vex_shlx_encodes() -> io::Result<()> {
        let mut writer = create_writer(5);
        let mut assembler = Assembler::new(&mut writer)?;

        assembler.op_reg_reg_reg(Reg32::ZAX, Register::R15, Register::R8, 0b10, 0xf7, 0b01)?;

        assert_eq!(assembler.start_offset(), 0);
        assert_eq!(assembler.current_offset(), 5);

        assembler.finish()?;

        assert_eq!(&*writer.into_inner(), &[0xc4, 0xc2, 0x39, 0xf7, 0xc7]);

        Ok(())
    }
}
