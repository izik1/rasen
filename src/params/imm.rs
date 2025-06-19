use super::{W8, W16, W32, W64, WWidth};

#[derive(Copy, Clone)]
pub struct Imm8(pub u8);

#[derive(Copy, Clone)]
pub struct Imm16(pub u16);

#[derive(Copy, Clone)]
pub struct Imm32(pub u32);

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum WritableImmediate {
    W8(u8),
    W16(u16),
    W32(u32),
    W64(u64),
}
