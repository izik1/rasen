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

pub trait WidthAtMost16: WWidth {}

impl WidthAtMost16 for W8 {}
impl WidthAtMost16 for W16 {}

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

impl Register {
    pub(crate) fn value(self) -> u8 {
        self as u8
    }

    pub(crate) fn needs_rex(self) -> bool {
        self.value() >= 8
    }

    pub(crate) fn writable(self) -> u8 {
        self.value() & 0b111
    }
}

pub trait GeneralRegister<Width>: Into<Register> {}

impl GeneralRegister<W8> for Register {}
impl GeneralRegister<W16> for Register {}
impl GeneralRegister<W32> for Register {}
impl GeneralRegister<W64> for Register {}

macro_rules! reg {
    ($reg:ident, $width:ident) => {
        /// A wrapper for [`Register`] That only implements [`GeneralRegister<$width>`], to make it more usable as a type param.
        #[derive(Copy, Clone)]
        pub struct $reg(pub Register);

        impl $reg {
            pub const ZAX: Self = Self(Register::Zax);
            pub const ZCX: Self = Self(Register::Zcx);
            pub const ZDX: Self = Self(Register::Zdx);
            pub const ZBX: Self = Self(Register::Zbx);
            pub const ZSP: Self = Self(Register::Zsp);
            pub const ZBP: Self = Self(Register::Zbp);
            pub const ZSI: Self = Self(Register::Zsi);
            pub const ZDI: Self = Self(Register::Zdi);
            pub const R8: Self = Self(Register::R8);
            pub const R9: Self = Self(Register::R9);
            pub const R10: Self = Self(Register::R10);
            pub const R11: Self = Self(Register::R11);
            pub const R12: Self = Self(Register::R12);
            pub const R13: Self = Self(Register::R13);
            pub const R14: Self = Self(Register::R14);
            pub const R15: Self = Self(Register::R15);
        }

        impl From<$reg> for Register {
            fn from(other: $reg) -> Self {
                other.0
            }
        }

        impl GeneralRegister<$width> for $reg {}
    };
}

reg!(Reg8, W8);
reg!(Reg16, W16);
reg!(Reg32, W32);
reg!(Reg64, W64);

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
