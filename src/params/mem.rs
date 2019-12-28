use crate::params::{Register, W16, W32, W64, W8, WWidth};

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
    pub fn new(r#mod: u8, reg: u8, rm: u8) -> Self {
        debug_assert!(r#mod < 4);
        debug_assert!(reg < 8);
        debug_assert!(rm < 8);

        Self((r#mod << 6) | ((reg & 0b111) << 3) | (rm & 0b111))
    }

    pub fn with_op(self, op: u8) -> Self {
        Self::new(self.r#mod(), op, self.rm())
    }

    pub fn with_reg(self, reg: u8) -> Self {
        Self::new(self.r#mod(), reg, self.rm())
    }

    pub fn r#mod(self) -> u8 {
        self.0 >> 6
    }

    pub fn reg(self) -> u8 {
        (self.0 >> 3) & 0b111
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn scale(self) -> u8 {
        self.0 >> 6
    }

    #[allow(dead_code)]
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

#[derive(Clone)]
pub struct Mem {
    base: Option<Register>,
    index: Option<Register>,
    displacement: i32,
    scale: Scale,
    has_index: bool,
    relative: bool,
    force_32x: bool,
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
            force_32x: false,
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
            force_32x: false,
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
                force_32x: false,
            })
        }
    }

    /// # Errors
    /// When [`index`] is [`Register::Zsp`], as Zsp can't be used as an index.
    pub fn zbp_index_scale_displacement(
        index: Register,
        scale: Scale,
        displacement: i32,
    ) -> Result<Self, ()> {
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
                force_32x: false,
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
    pub fn with_index_scale_displacement(
        index: Register,
        scale: Scale,
        displacement: i32,
    ) -> Result<Self, ()> {
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
                force_32x: false,
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
            force_32x: false,
        }
    }

    /// Forces the registers used as addresses to be the 32 bit versions instead of the 64 bit versions.
    pub fn x32(self) -> Self {
        Self {
            force_32x: true,
            ..self
        }
    }

    pub(crate) fn address_prefix(&self) -> Option<u8> {
        if self.force_32x {
            Some(0x67)
        } else {
            None
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
                // x86-64 encodes what would be [Zbp] pr [R13] as rip relative in this form, so we have to do [Zbp + 0].
                let mod_bits =
                    if self.displacement == 0 && base != Register::Zbp && base != Register::R13 {
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

        let base = self.base.map_or(SIB::NO_BASE, |base| (base as u8) % 8);

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

pub trait Memory<Width: WWidth>: Into<Mem> {}

impl Memory<W8> for Mem {}
impl Memory<W16> for Mem {}
impl Memory<W32> for Mem {}
impl Memory<W64> for Mem {}

// while the following _would_ look nicer with a macro, IDEs have trouble auto-completing it.

/// A wrapper for [`Mem`] That only implements [`Memory<W8>`], to make it more usable as a type param.
#[derive(Clone)]
#[repr(transparent)]
pub struct Mem8(pub Mem);

impl From<Mem8> for Mem {
    fn from(other: Mem8) -> Self {
        other.0
    }
}

impl Memory<W8> for Mem8 {}

/// A wrapper for [`Mem`] That only implements [`Memory<W16>`], to make it more usable as a type param.
#[derive(Clone)]
#[repr(transparent)]
pub struct Mem16(pub Mem);

impl From<Mem16> for Mem {
    fn from(other: Mem16) -> Self {
        other.0
    }
}

impl Memory<W16> for Mem16 {}

/// A wrapper for [`Mem`] That only implements [`Memory<W32>`], to make it more usable as a type param.
#[derive(Clone)]
#[repr(transparent)]
pub struct Mem32(pub Mem);

impl From<Mem32> for Mem {
    fn from(other: Mem32) -> Self {
        other.0
    }
}

impl Memory<W32> for Mem32 {}

/// A wrapper for [`Mem`] That only implements [`Memory<W64>`], to make it more usable as a type param.
#[derive(Clone)]
#[repr(transparent)]
pub struct Mem64(pub Mem);

impl From<Mem64> for Mem {
    fn from(other: Mem64) -> Self {
        other.0
    }
}

impl Memory<W64> for Mem64 {}
