use super::{W8, W16, W32, W64, WWidth};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
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

pub trait GeneralRegister<Width: WWidth>: Into<Register> {}

impl GeneralRegister<W8> for Register {}
impl GeneralRegister<W16> for Register {}
impl GeneralRegister<W32> for Register {}
impl GeneralRegister<W64> for Register {}

// While a macro _would_ be nice here, it kills ide auto-completion.

/// A wrapper for [`Register`] That only implements [`GeneralRegister<W8>`], to make it more usable as a type param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg8(pub Register);

impl Reg8 {
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

impl From<Reg8> for Register {
    fn from(other: Reg8) -> Self {
        other.0
    }
}

impl GeneralRegister<W8> for Reg8 {}

/// A wrapper for [`Register`] That only implements [`GeneralRegister<W16>`], to make it more usable as a type param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg16(pub Register);

impl Reg16 {
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

impl From<Reg16> for Register {
    fn from(other: Reg16) -> Self {
        other.0
    }
}

impl GeneralRegister<W16> for Reg16 {}

/// A wrapper for [`Register`] That only implements [`GeneralRegister<W32>`], to make it more usable as a type param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg32(pub Register);

impl Reg32 {
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

impl From<Reg32> for Register {
    fn from(other: Reg32) -> Self {
        other.0
    }
}

impl GeneralRegister<W32> for Reg32 {}

/// A wrapper for [`Register`] That only implements [`GeneralRegister<W64>`], to make it more usable as a type param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg64(pub Register);

impl Reg64 {
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

impl From<Reg64> for Register {
    fn from(other: Reg64) -> Self {
        other.0
    }
}

impl GeneralRegister<W64> for Reg64 {}
