pub mod imm;
pub mod mem;
pub mod reg;

pub struct W8;
pub struct W16;
pub struct W32;
pub struct W64;

pub use imm::{Immediate, WritableImmediate};
pub use mem::{Mem, Memory};
pub use reg::{GeneralRegister, Register};

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
