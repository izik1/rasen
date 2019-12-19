use crate::WritableImmediate;

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
    fn needs_rexb(&self) -> bool {
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

pub enum Displacement {
    Disp8(i8),
    Disp32(i32),
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Scale {
    X1 = 0b00,
    X2 = 0b01,
    X4 = 0b10,
    X8 = 0b11,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct ModRM(u8);

impl ModRM {
    fn new(r#mod: u8, reg: u8, rm: u8) -> Self {
        debug_assert!(r#mod < 4);
        debug_assert!(reg < 8);
        debug_assert!(rm < 8);

        Self((r#mod << 6) | ((reg & 0b111) << 3) | (rm & 0b111))
    }

    pub fn with_op(self, op: u8) -> Self {
        Self::new(self.r#mod(), op, self.rm())
    }

    pub fn r#mod(self) -> u8 {
        self.0 >> 6
    }

    pub fn reg(self) -> u8 {
        (self.0 >> 3) & 0b111
    }

    pub fn op(self) -> u8 {
        self.reg()
    }

    pub fn rm(self) -> u8 {
        self.0 & 0b111
    }

    pub fn has_sib(self) -> bool {
        self.rm() == Self::SIB_RM && self.r#mod() != 0b11
    }

    pub fn has_displacement(self) -> bool {
        self.r#mod() == 1 || self.r#mod() == 2
    }

    const SIB_RM: u8 = 0b100;
}

impl From<ModRM> for u8 {
    fn from(mod_rm: ModRM) -> Self {
        mod_rm.0
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct SIB(u8);

impl SIB {
    fn new(scale: u8, index: u8, base: u8) -> Self {
        debug_assert!(scale < 4);
        debug_assert!(index < 8);
        debug_assert!(base < 8);

        Self((scale << 6) | ((index & 0b111) << 3) | (base & 0b111))
    }

    pub fn scale(self) -> u8 {
        self.0 >> 6
    }

    pub fn index(self) -> u8 {
        (self.0 >> 3) & 0b111
    }

    pub fn base(self) -> u8 {
        self.0 & 0b111
    }

    pub fn has_displacement(self) -> bool {
        self.base() == Self::NO_BASE
    }

    const NO_INDEX: u8 = 0b100;
    const NO_BASE: u8 = 0b101;
}

impl From<SIB> for u8 {
    fn from(sib: SIB) -> Self {
        sib.0
    }
}

pub struct Mem {
    base: Option<Register>,
    index: Option<Register>,
    displacement: i32,
    scale: Scale,
    has_index: bool,
    relative: bool,
}

impl Mem {
    /// Creates a `Mem` instance with the given [`displacement`].
    /// # NOTE
    /// The displacement is automatically compressed to i8 if it's small enough.
    pub fn displacement(displacement: i32) -> Self {
        Self {
            base: None,
            index: None,
            displacement,
            scale: Scale::X1,
            has_index: false,
            relative: false,
        }
    }

    pub fn base(base: Register) -> Self {
        Self::base_displacement(base, 0)
    }

    pub fn base_displacement(base: Register, displacement: i32) -> Self {
        Self {
            base: Some(base),
            index: None,
            displacement,
            scale: Scale::X1,
            has_index: false,
            relative: false,
        }
    }

    /// # Errors
    /// When [`index`] is [`Register::Zsp`], as Zsp can't be used as an index.
    pub fn base_index(base: Register, index: Register) -> Result<Self, ()> {
        Self::base_index_scale(base, index, Scale::X1)
    }

    /// # Errors
    /// When [`index`] is [`Register::Zsp`], as Zsp can't be used as an index.
    pub fn base_index_scale(base: Register, index: Register, scale: Scale) -> Result<Self, ()> {
        if index == Register::Zsp {
            Err(())
        } else {
            Ok(Self {
                base: Some(base),
                index: Some(index),
                displacement: 0,
                scale,
                has_index: true,
                relative: false,
            })
        }
    }

    /// # Errors
    /// When [`index`] is [`Register::Zsp`], as Zsp can't be used as an index.
    pub fn zbp_index_scale_displacement(index: Register, scale: Scale, displacement: i32) -> Result<Self, ()> {
        if index == Register::Zsp {
            Err(())
        } else {
            Ok(Self {
                base: Some(Register::Zbp),
                index: Some(index),
                displacement,
                scale,
                has_index: true,
                relative: false,
            })
        }
    }

    /// # Errors
    /// When [`index`] is [`Register::Zsp`], as Zsp can't be used as an index.
    pub fn with_index(index: Register) -> Result<Self, ()> {
        Self::with_index_scale_displacement(index, Scale::X1, 0)
    }

    /// # Errors
    /// When [`index`] is [`Register::Zsp`], as Zsp can't be used as an index.
    pub fn with_index_scale(index: Register, scale: Scale) -> Result<Self, ()> {
        Self::with_index_scale_displacement(index, scale, 0)
    }

    pub fn with_index_displacement(index: Register, displacement: i32) -> Result<Self, ()> {
        Self::with_index_scale_displacement(index, Scale::X1, displacement)
    }

    /// # Errors
     /// When [`index`] is [`Register::Zsp`], as Zsp can't be used as an index.
    pub fn with_index_scale_displacement(index: Register, scale: Scale, displacement: i32) -> Result<Self, ()> {
        if index == Register::Zsp {
            Err(())
        } else {
            Ok(Self {
                base: None,
                index: Some(index),
                displacement,
                scale,
                has_index: true,
                relative: false,
            })
        }
    }

    pub fn relative() -> Self {
        Self::relative_displacement(0)
    }

    pub fn relative_displacement(displacement: i32) -> Self {
        Self {
            base: None,
            index: None,
            displacement,
            scale: Scale::X1,
            has_index: false,
            relative: true,
        }
    }

    pub(crate) fn encoded(&self) -> (ModRM, Option<SIB>, Option<Displacement>) {
        let mod_rm = self.mod_rm();
        if mod_rm.has_sib() {
            (mod_rm, Some(self.sib()), self.get_displacement())
        } else {
            (mod_rm, None, self.get_displacement())
        }
    }

    pub(crate) fn rex_byte(&self) -> u8 {
        let b = self.base.map_or(false, |it| it as u8 >= 8) as u8;
        let x = self.index.map_or(false, |it| it as u8 >= 8) as u8;

        if x != 0 || b != 0 {
            0b0100_0000 | (x << 1) | b
        } else {
            0
        }
    }

    pub(crate) fn mod_rm(&self) -> ModRM {
        match self.base {
            Some(base) => {
                // x86-64 encodes what would be [Zbp] pr [R13] as rip relative in this form, so have to do [Zbp + 0].
                let mod_bits = if self.displacement == 0 && base != Register::Zbp && base != Register::R13 {
                    0b00
                } else if self.displacement <= 0xff {
                    0b01
                } else {
                    0b10
                };

                // x86-64 encodes what would be [Zsp] or [R12] as a sib byte, so we need to use one.
                let base = if base == Register::Zsp || base == Register::R12 || self.has_index {
                    ModRM::SIB_RM
                } else {
                    (base as u8) % 8
                };

                ModRM::new(mod_bits, 0, base)
            }

            None if self.relative => ModRM::new(0, 0, 0b101),

            None => ModRM::new(0, 0, ModRM::SIB_RM),
        }
    }

    pub(crate) fn sib(&self) -> SIB {
        let index = self.index.map_or(SIB::NO_INDEX, |index| {
            // in release mode we just fall back to doing what X86 would do here... `none`
            // which is the same thing as when `index == None`.
            debug_assert_ne!(index, Register::Zsp);
            (index as u8) % 8
        });

        let base = self.base.map_or(SIB::NO_BASE, |base| {
             (base as u8) % 8
        });

        SIB::new(self.scale as u8, index, base)
    }

    pub(crate) fn get_displacement(&self) -> Option<Displacement> {
        let mod_rm = self.mod_rm();

        if mod_rm.has_displacement() || (mod_rm.has_sib() && self.sib().has_displacement()) {
            if mod_rm.r#mod() == 1 {
                Some(Displacement::Disp8(self.displacement as i8))
            } else {
                Some(Displacement::Disp32(self.displacement))
            }
        } else {
            None
        }
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
