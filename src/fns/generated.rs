use crate::params::{
    mem::Memory, GeneralRegister, Immediate, WWidth, WidthAtLeast16, WidthAtLeast32, WidthAtMost32,
    W16, W64, W8,
};
use crate::Assembler;
use std::io;

impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {
    pub fn adc_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0x14, 0x15)
    }

    pub fn add_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0x4, 0x5)
    }

    pub fn and_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0x24, 0x25)
    }

    pub fn cmp_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0x3c, 0x3d)
    }

    pub fn or_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0xc, 0xd)
    }

    pub fn sbb_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0x1c, 0x1d)
    }

    pub fn sub_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0x2c, 0x2d)
    }

    pub fn test_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0xa8, 0xa9)
    }

    pub fn xor_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {
        self.op_zax_imm(imm, 0x34, 0x35)
    }

    pub fn adc_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 2)
    }

    pub fn adc_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 2)
    }

    pub fn add_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 0)
    }

    pub fn add_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 0)
    }

    pub fn and_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 4)
    }

    pub fn and_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 4)
    }

    pub fn cmp_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 7)
    }

    pub fn cmp_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 7)
    }

    pub fn mov_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0xc6, 0xc7, 0)
    }

    pub fn mov_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0xc6, 0xc7, 0)
    }

    pub fn or_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 1)
    }

    pub fn or_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 1)
    }

    pub fn sbb_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 3)
    }

    pub fn sbb_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 3)
    }

    pub fn sub_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 5)
    }

    pub fn sub_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 5)
    }

    pub fn test_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0xf6, 0xf7, 0)
    }

    pub fn test_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0xf6, 0xf7, 0)
    }

    pub fn xor_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_reg_imm(reg, imm, 0x80, 0x81, 6)
    }

    pub fn xor_mem_imm<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: impl Immediate<Width>,
    ) -> io::Result<()> {
        self.op_mem_imm(mem, imm, 0x80, 0x81, 6)
    }

    pub fn bt_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xba, 0xba, 4, Some(0xf))
    }

    pub fn bt_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xba, 0xba, 4, Some(0xf))
    }

    pub fn btc_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xba, 0xba, 7, Some(0xf))
    }

    pub fn btc_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xba, 0xba, 7, Some(0xf))
    }

    pub fn btr_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xba, 0xba, 6, Some(0xf))
    }

    pub fn btr_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xba, 0xba, 6, Some(0xf))
    }

    pub fn bts_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xba, 0xba, 5, Some(0xf))
    }

    pub fn bts_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xba, 0xba, 5, Some(0xf))
    }

    pub fn rcl_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 2, None)
    }

    pub fn rcl_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 2, None)
    }

    pub fn rcr_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 3, None)
    }

    pub fn rcr_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 3, None)
    }

    pub fn rol_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 0, None)
    }

    pub fn rol_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 0, None)
    }

    pub fn ror_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 1, None)
    }

    pub fn ror_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 1, None)
    }

    pub fn sal_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 4, None)
    }

    pub fn sal_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 4, None)
    }

    pub fn sar_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 7, None)
    }

    pub fn sar_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 7, None)
    }

    pub fn shl_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 4, None)
    }

    pub fn shl_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 4, None)
    }

    pub fn shr_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: u8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm, 0xc0, 0xc1, 5, None)
    }

    pub fn shr_mem_imm8<Width: WWidth, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: u8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm, 0xc0, 0xc1, 5, None)
    }

    pub fn adc_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm as u8, 0x83, 0x83, 2, None)
    }

    pub fn adc_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: i8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm as u8, 0x83, 0x83, 2, None)
    }

    pub fn add_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm as u8, 0x83, 0x83, 0, None)
    }

    pub fn add_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: i8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm as u8, 0x83, 0x83, 0, None)
    }

    pub fn cmp_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm as u8, 0x83, 0x83, 7, None)
    }

    pub fn cmp_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: i8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm as u8, 0x83, 0x83, 7, None)
    }

    pub fn or_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm as u8, 0x83, 0x83, 1, None)
    }

    pub fn or_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: i8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm as u8, 0x83, 0x83, 1, None)
    }

    pub fn sbb_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm as u8, 0x83, 0x83, 3, None)
    }

    pub fn sbb_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: i8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm as u8, 0x83, 0x83, 3, None)
    }

    pub fn sub_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm as u8, 0x83, 0x83, 5, None)
    }

    pub fn sub_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: i8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm as u8, 0x83, 0x83, 5, None)
    }

    pub fn xor_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
        imm: i8,
    ) -> io::Result<()> {
        self.op_reg_imm8(reg, imm as u8, 0x83, 0x83, 6, None)
    }

    pub fn xor_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(
        &mut self,
        mem: M,
        imm: i8,
    ) -> io::Result<()> {
        self.op_mem_imm8(mem, imm as u8, 0x83, 0x83, 6, None)
    }

    pub fn adc_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x12, 0x13, None)
    }

    pub fn adc_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x12, 0x13, None)
    }

    pub fn add_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x2, 0x3, None)
    }

    pub fn add_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x2, 0x3, None)
    }

    pub fn and_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x22, 0x23, None)
    }

    pub fn and_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x22, 0x23, None)
    }

    pub fn bsf_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xbc, 0xbc, Some(0xf))
    }

    pub fn bsf_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xbc, 0xbc, Some(0xf))
    }

    pub fn bsr_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xbd, 0xbd, Some(0xf))
    }

    pub fn bsr_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xbd, 0xbd, Some(0xf))
    }

    pub fn cmova_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x47, 0x47, Some(0xf))
    }

    pub fn cmova_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x47, 0x47, Some(0xf))
    }

    pub fn cmovae_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x43, 0x43, Some(0xf))
    }

    pub fn cmovae_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x43, 0x43, Some(0xf))
    }

    pub fn cmovb_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x42, 0x42, Some(0xf))
    }

    pub fn cmovb_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x42, 0x42, Some(0xf))
    }

    pub fn cmovbe_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x46, 0x46, Some(0xf))
    }

    pub fn cmovbe_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x46, 0x46, Some(0xf))
    }

    pub fn cmovc_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x42, 0x42, Some(0xf))
    }

    pub fn cmovc_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x42, 0x42, Some(0xf))
    }

    pub fn cmove_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x44, 0x44, Some(0xf))
    }

    pub fn cmove_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x44, 0x44, Some(0xf))
    }

    pub fn cmovg_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4f, 0x4f, Some(0xf))
    }

    pub fn cmovg_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4f, 0x4f, Some(0xf))
    }

    pub fn cmovge_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4d, 0x4d, Some(0xf))
    }

    pub fn cmovge_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4d, 0x4d, Some(0xf))
    }

    pub fn cmovl_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4c, 0x4c, Some(0xf))
    }

    pub fn cmovl_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4c, 0x4c, Some(0xf))
    }

    pub fn cmovle_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4e, 0x4e, Some(0xf))
    }

    pub fn cmovle_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4e, 0x4e, Some(0xf))
    }

    pub fn cmovna_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x46, 0x46, Some(0xf))
    }

    pub fn cmovna_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x46, 0x46, Some(0xf))
    }

    pub fn cmovnae_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x42, 0x42, Some(0xf))
    }

    pub fn cmovnae_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x42, 0x42, Some(0xf))
    }

    pub fn cmovnb_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x43, 0x43, Some(0xf))
    }

    pub fn cmovnb_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x43, 0x43, Some(0xf))
    }

    pub fn cmovnbe_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x47, 0x47, Some(0xf))
    }

    pub fn cmovnbe_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x47, 0x47, Some(0xf))
    }

    pub fn cmovnc_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x43, 0x43, Some(0xf))
    }

    pub fn cmovnc_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x43, 0x43, Some(0xf))
    }

    pub fn cmovne_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x45, 0x45, Some(0xf))
    }

    pub fn cmovne_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x45, 0x45, Some(0xf))
    }

    pub fn cmovng_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4e, 0x4e, Some(0xf))
    }

    pub fn cmovng_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4e, 0x4e, Some(0xf))
    }

    pub fn cmovnge_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4c, 0x4c, Some(0xf))
    }

    pub fn cmovnge_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4c, 0x4c, Some(0xf))
    }

    pub fn cmovnl_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4d, 0x4d, Some(0xf))
    }

    pub fn cmovnl_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4d, 0x4d, Some(0xf))
    }

    pub fn cmovnle_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4f, 0x4f, Some(0xf))
    }

    pub fn cmovnle_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4f, 0x4f, Some(0xf))
    }

    pub fn cmovno_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x41, 0x41, Some(0xf))
    }

    pub fn cmovno_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x41, 0x41, Some(0xf))
    }

    pub fn cmovnp_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4b, 0x4b, Some(0xf))
    }

    pub fn cmovnp_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4b, 0x4b, Some(0xf))
    }

    pub fn cmovns_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x49, 0x49, Some(0xf))
    }

    pub fn cmovns_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x49, 0x49, Some(0xf))
    }

    pub fn cmovnz_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x45, 0x45, Some(0xf))
    }

    pub fn cmovnz_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x45, 0x45, Some(0xf))
    }

    pub fn cmovo_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x40, 0x40, Some(0xf))
    }

    pub fn cmovo_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x40, 0x40, Some(0xf))
    }

    pub fn cmovp_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4a, 0x4a, Some(0xf))
    }

    pub fn cmovp_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4a, 0x4a, Some(0xf))
    }

    pub fn cmovpe_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4a, 0x4a, Some(0xf))
    }

    pub fn cmovpe_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4a, 0x4a, Some(0xf))
    }

    pub fn cmovpo_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x4b, 0x4b, Some(0xf))
    }

    pub fn cmovpo_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x4b, 0x4b, Some(0xf))
    }

    pub fn cmovs_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x48, 0x48, Some(0xf))
    }

    pub fn cmovs_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x48, 0x48, Some(0xf))
    }

    pub fn cmovz_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x44, 0x44, Some(0xf))
    }

    pub fn cmovz_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x44, 0x44, Some(0xf))
    }

    pub fn cmp_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x3a, 0x3b, None)
    }

    pub fn cmp_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x3a, 0x3b, None)
    }

    pub fn imul_reg_mem<Width: WidthAtLeast16, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xaf, 0xaf, Some(0xf))
    }

    pub fn imul_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xaf, 0xaf, Some(0xf))
    }

    pub fn lar_reg_mem<Width: WidthAtLeast16 + WidthAtMost32, R, M>(
        &mut self,
        reg: R,
        mem: M,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x2, 0x2, Some(0xf))
    }

    pub fn lar_reg_reg<Width: WidthAtLeast16 + WidthAtMost32, R>(
        &mut self,
        reg1: R,
        reg2: R,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x2, 0x2, Some(0xf))
    }

    pub fn lea_reg_mem<Width: WidthAtLeast32, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x8d, 0x8d, None)
    }

    pub fn lea_reg_reg<Width: WidthAtLeast32, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x8d, 0x8d, None)
    }

    pub fn lsl_reg_mem<Width: WidthAtLeast16 + WidthAtMost32, R, M>(
        &mut self,
        reg: R,
        mem: M,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x3, 0x3, Some(0xf))
    }

    pub fn lsl_reg_reg<Width: WidthAtLeast16 + WidthAtMost32, R>(
        &mut self,
        reg1: R,
        reg2: R,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x3, 0x3, Some(0xf))
    }

    pub fn mov_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x8a, 0x8b, None)
    }

    pub fn mov_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x8a, 0x8b, None)
    }

    pub fn or_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xa, 0xb, None)
    }

    pub fn or_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xa, 0xb, None)
    }

    pub fn sbb_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x1a, 0x1b, None)
    }

    pub fn sbb_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x1a, 0x1b, None)
    }

    pub fn sub_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x2a, 0x2b, None)
    }

    pub fn sub_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x2a, 0x2b, None)
    }

    pub fn xchg_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x86, 0x87, None)
    }

    pub fn xchg_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x86, 0x87, None)
    }

    pub fn xor_reg_mem<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x32, 0x33, None)
    }

    pub fn xor_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x32, 0x33, None)
    }

    pub fn adc_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x10, 0x11, None)
    }

    pub fn add_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x0, 0x1, None)
    }

    pub fn and_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x20, 0x21, None)
    }

    pub fn bt_mem_reg<Width: WidthAtLeast16, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xa3, 0xa3, Some(0xf))
    }

    pub fn bt_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xa3, 0xa3, Some(0xf))
    }

    pub fn btc_mem_reg<Width: WidthAtLeast16, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xbb, 0xbb, Some(0xf))
    }

    pub fn btc_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xbb, 0xbb, Some(0xf))
    }

    pub fn btr_mem_reg<Width: WidthAtLeast16, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xb3, 0xb3, Some(0xf))
    }

    pub fn btr_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xb3, 0xb3, Some(0xf))
    }

    pub fn bts_mem_reg<Width: WidthAtLeast16, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xab, 0xab, Some(0xf))
    }

    pub fn bts_reg_reg<Width: WidthAtLeast16, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xab, 0xab, Some(0xf))
    }

    pub fn cmp_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x38, 0x39, None)
    }

    pub fn mov_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x88, 0x89, None)
    }

    pub fn movnti_mem_reg<Width: WidthAtLeast32, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xc3, 0xc3, Some(0xf))
    }

    pub fn movnti_reg_reg<Width: WidthAtLeast32, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xc3, 0xc3, Some(0xf))
    }

    pub fn or_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x8, 0x9, None)
    }

    pub fn sbb_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x18, 0x19, None)
    }

    pub fn sub_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x28, 0x29, None)
    }

    pub fn test_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x84, 0x85, None)
    }

    pub fn test_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0x84, 0x85, None)
    }

    pub fn xadd_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0xc0, 0xc1, Some(0xf))
    }

    pub fn xadd_reg_reg<Width: WWidth, R>(&mut self, reg1: R, reg2: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
    {
        self.op_reg_reg(reg1, reg2, 0xc0, 0xc1, Some(0xf))
    }

    pub fn xchg_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x86, 0x87, None)
    }

    pub fn xor_mem_reg<Width: WWidth, R, M>(&mut self, mem: M, reg: R) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_rm_mr(reg, mem, 0x30, 0x31, None)
    }

    pub fn clc(&mut self) -> io::Result<()> {
        self.op_no_operands(0xf8, None)
    }

    pub fn cld(&mut self) -> io::Result<()> {
        self.op_no_operands(0xfc, None)
    }

    pub fn cli(&mut self) -> io::Result<()> {
        self.op_no_operands(0xfa, None)
    }

    pub fn clts(&mut self) -> io::Result<()> {
        self.op_no_operands(0x6, Some(0xf))
    }

    pub fn cmc(&mut self) -> io::Result<()> {
        self.op_no_operands(0xf5, None)
    }

    pub fn emms(&mut self) -> io::Result<()> {
        self.op_no_operands(0x77, Some(0xf))
    }

    pub fn femms(&mut self) -> io::Result<()> {
        self.op_no_operands(0xe, Some(0xf))
    }

    pub fn fwait(&mut self) -> io::Result<()> {
        self.op_no_operands(0xdb, None)
    }

    pub fn getsec(&mut self) -> io::Result<()> {
        self.op_no_operands(0x37, Some(0xf))
    }

    pub fn hlt(&mut self) -> io::Result<()> {
        self.op_no_operands(0xf4, None)
    }

    pub fn int3(&mut self) -> io::Result<()> {
        self.op_no_operands(0xcc, None)
    }

    pub fn invd(&mut self) -> io::Result<()> {
        self.op_no_operands(0x8, Some(0xf))
    }

    pub fn iret(&mut self) -> io::Result<()> {
        self.op_no_operands(0xcf, None)
    }

    pub fn iretd(&mut self) -> io::Result<()> {
        self.op_no_operands(0xcf, None)
    }

    pub fn iretq(&mut self) -> io::Result<()> {
        self.op_no_operands(0xcf, None)
    }

    pub fn iretw(&mut self) -> io::Result<()> {
        self.op_no_operands(0xcf, None)
    }

    pub fn leave(&mut self) -> io::Result<()> {
        self.op_no_operands(0xc9, None)
    }

    pub fn nop(&mut self) -> io::Result<()> {
        self.op_no_operands(0x90, None)
    }

    pub fn popf(&mut self) -> io::Result<()> {
        self.op_no_operands(0x9d, None)
    }

    pub fn popfq(&mut self) -> io::Result<()> {
        self.op_no_operands(0x9d, None)
    }

    pub fn pushf(&mut self) -> io::Result<()> {
        self.op_no_operands(0x9c, None)
    }

    pub fn pushfq(&mut self) -> io::Result<()> {
        self.op_no_operands(0x9c, None)
    }

    pub fn ret(&mut self) -> io::Result<()> {
        self.op_no_operands(0xc3, None)
    }

    pub fn stc(&mut self) -> io::Result<()> {
        self.op_no_operands(0xf9, None)
    }

    pub fn std(&mut self) -> io::Result<()> {
        self.op_no_operands(0xfd, None)
    }

    pub fn sti(&mut self) -> io::Result<()> {
        self.op_no_operands(0xfb, None)
    }

    pub fn syscall(&mut self) -> io::Result<()> {
        self.op_no_operands(0x5, Some(0xf))
    }

    pub fn sysenter(&mut self) -> io::Result<()> {
        self.op_no_operands(0x34, Some(0xf))
    }

    pub fn sysexit(&mut self) -> io::Result<()> {
        self.op_no_operands(0x35, Some(0xf))
    }

    pub fn sysexit64(&mut self) -> io::Result<()> {
        self.op_no_operands(0x35, Some(0xf))
    }

    pub fn sysret(&mut self) -> io::Result<()> {
        self.op_no_operands(0x7, Some(0xf))
    }

    pub fn sysret64(&mut self) -> io::Result<()> {
        self.op_no_operands(0x7, Some(0xf))
    }

    pub fn ud2(&mut self) -> io::Result<()> {
        self.op_no_operands(0xb, Some(0xf))
    }

    pub fn wait(&mut self) -> io::Result<()> {
        self.op_no_operands(0xdb, None)
    }

    pub fn wbinvd(&mut self) -> io::Result<()> {
        self.op_no_operands(0x9, Some(0xf))
    }

    pub fn xlatb(&mut self) -> io::Result<()> {
        self.op_no_operands(0xd7, None)
    }

    pub fn bextr_reg_mem_reg<Width: WidthAtLeast32, R, M>(
        &mut self,
        rd: R,
        mem: M,
        rs: R,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_reg_mem_reg(rd, mem, rs, 0x2, 0xf7, 0x0)
    }

    pub fn bextr_reg_reg_reg<Width: WidthAtLeast32, R: GeneralRegister<Width>>(
        &mut self,
        rd: R,
        rs1: R,
        rs2: R,
    ) -> io::Result<()> {
        self.op_reg_reg_reg(rd, rs1, rs2, 0x2, 0xf7, 0x0)
    }

    pub fn bzhi_reg_mem_reg<Width: WidthAtLeast32, R, M>(
        &mut self,
        rd: R,
        mem: M,
        rs: R,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_reg_mem_reg(rd, mem, rs, 0x2, 0xf5, 0x0)
    }

    pub fn bzhi_reg_reg_reg<Width: WidthAtLeast32, R: GeneralRegister<Width>>(
        &mut self,
        rd: R,
        rs1: R,
        rs2: R,
    ) -> io::Result<()> {
        self.op_reg_reg_reg(rd, rs1, rs2, 0x2, 0xf5, 0x0)
    }

    pub fn sarx_reg_mem_reg<Width: WidthAtLeast32, R, M>(
        &mut self,
        rd: R,
        mem: M,
        rs: R,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_reg_mem_reg(rd, mem, rs, 0x2, 0xf7, 0x2)
    }

    pub fn sarx_reg_reg_reg<Width: WidthAtLeast32, R: GeneralRegister<Width>>(
        &mut self,
        rd: R,
        rs1: R,
        rs2: R,
    ) -> io::Result<()> {
        self.op_reg_reg_reg(rd, rs1, rs2, 0x2, 0xf7, 0x2)
    }

    pub fn shlx_reg_mem_reg<Width: WidthAtLeast32, R, M>(
        &mut self,
        rd: R,
        mem: M,
        rs: R,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_reg_mem_reg(rd, mem, rs, 0x2, 0xf7, 0x1)
    }

    pub fn shlx_reg_reg_reg<Width: WidthAtLeast32, R: GeneralRegister<Width>>(
        &mut self,
        rd: R,
        rs1: R,
        rs2: R,
    ) -> io::Result<()> {
        self.op_reg_reg_reg(rd, rs1, rs2, 0x2, 0xf7, 0x1)
    }

    pub fn shrx_reg_mem_reg<Width: WidthAtLeast32, R, M>(
        &mut self,
        rd: R,
        mem: M,
        rs: R,
    ) -> io::Result<()>
    where
        R: GeneralRegister<Width>,
        M: Memory<Width>,
    {
        self.op_reg_mem_reg(rd, mem, rs, 0x2, 0xf7, 0x3)
    }

    pub fn shrx_reg_reg_reg<Width: WidthAtLeast32, R: GeneralRegister<Width>>(
        &mut self,
        rd: R,
        rs1: R,
        rs2: R,
    ) -> io::Result<()> {
        self.op_reg_reg_reg(rd, rs1, rs2, 0x2, 0xf7, 0x3)
    }

    pub fn call_reg64<R: GeneralRegister<W64>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0xff, 0xff, Some(0x2), None)
    }

    pub fn call_mem64<M: Memory<W64>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0xff, 0xff, Some(0x2), None)
    }

    pub fn dec_reg<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0xfe, 0xff, Some(0x1), None)
    }

    pub fn dec_mem<Width: WWidth, M: Memory<Width>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0xfe, 0xff, Some(0x1), None)
    }

    pub fn inc_reg<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0xfe, 0xff, Some(0x0), None)
    }

    pub fn inc_mem<Width: WWidth, M: Memory<Width>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0xfe, 0xff, Some(0x0), None)
    }

    pub fn lldt_reg16<R: GeneralRegister<W16>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x0, 0x0, Some(0x2), Some(0xf))
    }

    pub fn lldt_mem16<M: Memory<W16>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x0, 0x0, Some(0x2), Some(0xf))
    }

    pub fn lmsw_reg16<R: GeneralRegister<W16>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x1, 0x1, Some(0x6), Some(0xf))
    }

    pub fn lmsw_mem16<M: Memory<W16>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x1, 0x1, Some(0x6), Some(0xf))
    }

    pub fn ltr_reg16<R: GeneralRegister<W16>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x0, 0x0, Some(0x3), Some(0xf))
    }

    pub fn ltr_mem16<M: Memory<W16>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x0, 0x0, Some(0x3), Some(0xf))
    }

    pub fn neg_reg<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0xf6, 0xf7, Some(0x3), None)
    }

    pub fn neg_mem<Width: WWidth, M: Memory<Width>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0xf6, 0xf7, Some(0x3), None)
    }

    pub fn nop_reg<Width: WidthAtLeast16 + WidthAtMost32, R: GeneralRegister<Width>>(
        &mut self,
        reg: R,
    ) -> io::Result<()> {
        self.op_reg(reg, 0x1f, 0x1f, Some(0x0), Some(0xf))
    }

    pub fn nop_mem<Width: WidthAtLeast16 + WidthAtMost32, M: Memory<Width>>(
        &mut self,
        mem: M,
    ) -> io::Result<()> {
        self.op_mem(mem, 0x1f, 0x1f, Some(0x0), Some(0xf))
    }

    pub fn not_reg<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0xf6, 0xf7, Some(0x2), None)
    }

    pub fn not_mem<Width: WWidth, M: Memory<Width>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0xf6, 0xf7, Some(0x2), None)
    }

    pub fn seta_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x97, 0x97, None, Some(0xf))
    }

    pub fn seta_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x97, 0x97, None, Some(0xf))
    }

    pub fn setae_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x93, 0x93, None, Some(0xf))
    }

    pub fn setae_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x93, 0x93, None, Some(0xf))
    }

    pub fn setb_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x92, 0x92, None, Some(0xf))
    }

    pub fn setb_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x92, 0x92, None, Some(0xf))
    }

    pub fn setbe_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x96, 0x96, None, Some(0xf))
    }

    pub fn setbe_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x96, 0x96, None, Some(0xf))
    }

    pub fn setc_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x92, 0x92, None, Some(0xf))
    }

    pub fn setc_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x92, 0x92, None, Some(0xf))
    }

    pub fn sete_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x94, 0x94, None, Some(0xf))
    }

    pub fn sete_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x94, 0x94, None, Some(0xf))
    }

    pub fn setg_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9f, 0x9f, None, Some(0xf))
    }

    pub fn setg_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9f, 0x9f, None, Some(0xf))
    }

    pub fn setge_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9d, 0x9d, None, Some(0xf))
    }

    pub fn setge_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9d, 0x9d, None, Some(0xf))
    }

    pub fn setl_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9c, 0x9c, None, Some(0xf))
    }

    pub fn setl_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9c, 0x9c, None, Some(0xf))
    }

    pub fn setle_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9e, 0x9e, None, Some(0xf))
    }

    pub fn setle_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9e, 0x9e, None, Some(0xf))
    }

    pub fn setna_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x96, 0x96, None, Some(0xf))
    }

    pub fn setna_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x96, 0x96, None, Some(0xf))
    }

    pub fn setnae_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x92, 0x92, None, Some(0xf))
    }

    pub fn setnae_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x92, 0x92, None, Some(0xf))
    }

    pub fn setnb_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x93, 0x93, None, Some(0xf))
    }

    pub fn setnb_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x93, 0x93, None, Some(0xf))
    }

    pub fn setnbe_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x97, 0x97, None, Some(0xf))
    }

    pub fn setnbe_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x97, 0x97, None, Some(0xf))
    }

    pub fn setnc_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x93, 0x93, None, Some(0xf))
    }

    pub fn setnc_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x93, 0x93, None, Some(0xf))
    }

    pub fn setne_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x95, 0x95, None, Some(0xf))
    }

    pub fn setne_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x95, 0x95, None, Some(0xf))
    }

    pub fn setng_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9e, 0x9e, None, Some(0xf))
    }

    pub fn setng_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9e, 0x9e, None, Some(0xf))
    }

    pub fn setnge_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9c, 0x9c, None, Some(0xf))
    }

    pub fn setnge_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9c, 0x9c, None, Some(0xf))
    }

    pub fn setnl_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9d, 0x9d, None, Some(0xf))
    }

    pub fn setnl_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9d, 0x9d, None, Some(0xf))
    }

    pub fn setnle_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9f, 0x9f, None, Some(0xf))
    }

    pub fn setnle_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9f, 0x9f, None, Some(0xf))
    }

    pub fn setno_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x91, 0x91, None, Some(0xf))
    }

    pub fn setno_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x91, 0x91, None, Some(0xf))
    }

    pub fn setnp_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9b, 0x9b, None, Some(0xf))
    }

    pub fn setnp_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9b, 0x9b, None, Some(0xf))
    }

    pub fn setns_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x99, 0x99, None, Some(0xf))
    }

    pub fn setns_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x99, 0x99, None, Some(0xf))
    }

    pub fn setnz_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x95, 0x95, None, Some(0xf))
    }

    pub fn setnz_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x95, 0x95, None, Some(0xf))
    }

    pub fn seto_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x90, 0x90, None, Some(0xf))
    }

    pub fn seto_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x90, 0x90, None, Some(0xf))
    }

    pub fn setp_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9a, 0x9a, None, Some(0xf))
    }

    pub fn setp_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9a, 0x9a, None, Some(0xf))
    }

    pub fn setpe_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9a, 0x9a, None, Some(0xf))
    }

    pub fn setpe_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9a, 0x9a, None, Some(0xf))
    }

    pub fn setpo_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x9b, 0x9b, None, Some(0xf))
    }

    pub fn setpo_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x9b, 0x9b, None, Some(0xf))
    }

    pub fn sets_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x98, 0x98, None, Some(0xf))
    }

    pub fn sets_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x98, 0x98, None, Some(0xf))
    }

    pub fn setz_reg8<R: GeneralRegister<W8>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x94, 0x94, None, Some(0xf))
    }

    pub fn setz_mem8<M: Memory<W8>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x94, 0x94, None, Some(0xf))
    }

    pub fn verr_reg16<R: GeneralRegister<W16>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x0, 0x0, Some(0x4), Some(0xf))
    }

    pub fn verr_mem16<M: Memory<W16>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x0, 0x0, Some(0x4), Some(0xf))
    }

    pub fn verw_reg16<R: GeneralRegister<W16>>(&mut self, reg: R) -> io::Result<()> {
        self.op_reg(reg, 0x0, 0x0, Some(0x5), Some(0xf))
    }

    pub fn verw_mem16<M: Memory<W16>>(&mut self, mem: M) -> io::Result<()> {
        self.op_mem(mem, 0x0, 0x0, Some(0x5), Some(0xf))
    }
}
