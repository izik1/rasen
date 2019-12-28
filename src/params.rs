use crate::WritableImmediate;

pub mod mem;
pub mod reg;

pub use reg::{GeneralRegister, Reg16, Reg32, Reg64, Reg8, Register};

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
    const IS_W64: bool = false;
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
    const IS_W64: bool = true;
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

pub trait WidthAtMost16: WWidth {}

impl WidthAtMost16 for W8 {}
impl WidthAtMost16 for W16 {}

pub struct Imm8(pub u8);

pub struct Imm16(pub u16);

pub struct Imm32(pub u32);

pub struct Imm64(pub i32);

pub trait Immediate<Width: WWidth> {
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
