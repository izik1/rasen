use crate::params::{GeneralRegister, Immediate, WWidth, WidthAtLeast16};
use crate::Assembler;
use std::io;

include!(concat!(env!("OUT_DIR"), "/fns.rs"));

// todo: maybe redirect to `op_reg_imm` if `Width == 8` instead of WidthAtLeast16 trait bound?
//  above is not doable without const_generics, because `WidthAtLeast16` != W8,
//  and `Width::IS_W8` doesn't prove (to the compiler anyway) that `Width == W8`


impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {
    // xor_hi8_imm<Width>(Hi8Bit, u8)
    // xor_mem_imm<Width>(Mem<Width>, Immediate<Width>)
    // xor_mem_sximm8<Width>(Mem<Width>, i8)
    // xor_mem_reg<Width>(Mem<Width>, Register<Width>)
    // xor_mem_hi8(Mem<W8>, Hi8Bit)
    // xor_reg_mem<Width>(Register<Width>, Mem<Width>)
    // xor_hi8_mem(Hi8Bit, Mem<W8>)
}
