use crate::WritableImmediate;

pub mod mem;

pub struct W8;
pub struct W16;
pub struct W32;
pub struct W64;

mod private {
    pub trait Sealed {}

    impl Sealed for super::W8 {}
    impl Sealed for super::W16 {}
    impl Sealed for super::W32 {}
    impl Sealed for super::W64 {}
}

pub trait WWidth: private::Sealed {
    const HAS_REXW: bool = false;
    const IS_W8: bool = false;
    const IS_W16: bool = false;
}

impl WWidth for W8 {
    const IS_W8: bool = true;
}

impl WWidth for W16 {
    const IS_W16: bool = true;
}

impl WWidth for W32 {}

impl WWidth for W64 {
    const HAS_REXW: bool = true;
}

pub trait WidthAtLeast16: WWidth {}

impl WidthAtLeast16 for W16 {}
impl WidthAtLeast16 for W32 {}
impl WidthAtLeast16 for W64 {}

pub trait WidthAtLeast32: WWidth {}

impl WidthAtLeast32 for W32 {}
impl WidthAtLeast32 for W64 {}

pub trait WidthAtMost32: WWidth {}

impl WidthAtMost32 for W8 {}
impl WidthAtMost32 for W16 {}
impl WidthAtMost32 for W32 {}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Register {
    Zax = 0,
    Zcx = 1,
    Zdx = 2,
    Zbx = 3,
    Zsp = 4,
    Zbp = 5,
    Zsi = 6,
    Zdi = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

pub trait GeneralRegister<Width> {
    fn value(&self) -> u8;
    fn needs_rex(&self) -> bool {
        self.value() >= 8
    }
}

impl GeneralRegister<W8> for Register {
    fn value(&self) -> u8 {
        *self as u8
    }
}

impl GeneralRegister<W16> for Register {
    fn value(&self) -> u8 {
        *self as u8
    }
}

impl GeneralRegister<W32> for Register {
    fn value(&self) -> u8 {
        *self as u8
    }
}

impl GeneralRegister<W64> for Register {
    fn value(&self) -> u8 {
        *self as u8
    }
}

pub struct Imm8(pub u8);

pub struct Imm16(pub u16);

pub struct Imm32(pub u32);

pub struct Imm64(pub i32);

pub trait Immediate<Width> {
    fn as_writable(&self) -> WritableImmediate;
}

impl Immediate<W8> for Imm8 {
    fn as_writable(&self) -> WritableImmediate {
        WritableImmediate::W8(self.0)
    }
}

impl Immediate<W16> for Imm16 {
    fn as_writable(&self) -> WritableImmediate {
        WritableImmediate::W16(self.0)
    }
}

impl Immediate<W32> for Imm32 {
    fn as_writable(&self) -> WritableImmediate {
        WritableImmediate::W32(self.0)
    }
}

impl Immediate<W64> for Imm64 {
    fn as_writable(&self) -> WritableImmediate {
        WritableImmediate::W32(self.0 as u32)
    }
}
