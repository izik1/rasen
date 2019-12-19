use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

trait Opcode {
    const FILE_NAME: &'static str;

    fn path() -> PathBuf {
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&root_dir)
            .join("asm_instrs")
            .join(Self::FILE_NAME)
    }
}

#[derive(Debug, serde_derive::Deserialize, Clone)]
struct Op {
    name: String,
    op8: u8,
    op: u8,
    rm: Option<u8>,
}

#[derive(Debug, serde_derive::Deserialize, Clone)]
struct WidthAtLeast16Op {
    name: String,
    op: u8,
    rm: Option<u8>,
}

#[derive(Debug, serde_derive::Deserialize)]
struct Ops {
    zax_imm: Vec<Op>,
    rm_imm: Vec<Op>,
    rm_sximm8: Vec<WidthAtLeast16Op>,
    reg_mem: Vec<Op>,
    mem_reg: Vec<Op>,
}

impl Opcode for Op {
    const FILE_NAME: &'static str = "ops.json";
}

fn write_op_zax_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {{
        self.op_zax_imm(imm, {op8:#02x?}, {op:#02x?})
    }}
"#, name=op.name, op=op.op, op8=op.op8).unwrap();
}

fn write_op_reg_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R, imm: impl Immediate<Width>) -> io::Result<()> {{
        self.op_reg_imm(reg, imm, {op8:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, op8=op.op8, rm=op.rm.unwrap()).unwrap();
}

fn write_op_mem_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_mem_imm<Width: WWidth, M: Memory<Width>>(&mut self, mem: M, imm: impl Immediate<Width>) -> io::Result<()> {{
        self.op_mem_imm(mem, imm, {op8:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, op8=op.op8, rm=op.rm.unwrap()).unwrap();
}

fn write_op_reg_sximm8(f: &mut File, op: WidthAtLeast16Op) {
    writeln!(f, r#"    pub fn {name}_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(&mut self, reg: R, imm: i8) -> io::Result<()> {{
        self.op_reg_sximm8(reg, {op:#02x?}, {rm}, imm)
    }}
"#, name=op.name, op=op.op, rm=op.rm.unwrap()).unwrap();
}

fn write_op_mem_sximm8(f: &mut File, op: WidthAtLeast16Op) {
    writeln!(f, r#"    pub fn {name}_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(&mut self, mem: M, imm: i8) -> io::Result<()> {{
        self.op_mem_sximm8(mem, {op:#02x?}, {rm}, imm)
    }}
"#, name=op.name, op=op.op, rm=op.rm.unwrap()).unwrap();
}

fn write_op_rm_mr(f: &mut File, op: Op, group: &str) {
    writeln!(f, r#"    pub fn {name}_{group}<Width: WWidth, R, M>(&mut self, reg: R, mem: M) -> io::Result<()> where R: GeneralRegister<Width>, M: Memory<M> {{
        self.op_rm_mr(reg, mem, {op8:#02x?}, {op:#02x?})
    }}
"#, name=op.name, op=op.op, op8=op.op8, group=group).unwrap();
}

fn write_ops(f: &mut File) {
    writeln!(f, "impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {{").unwrap();

    let ops: Ops = serde_json::from_reader(File::open(Op::path()).unwrap()).unwrap();

    for op in ops.zax_imm {
        write_op_zax_imm(f, op);
    }

    for op in ops.rm_imm {
        write_op_reg_imm(f, op.clone());
        write_op_mem_imm(f, op);
    }

    for op in ops.rm_sximm8 {
        write_op_reg_sximm8(f, op.clone());
        write_op_mem_sximm8(f, op);
    }

    for op in ops.reg_mem {
        write_op_rm_mr(f, op, "reg_mem");
    }

    for op in ops.mem_reg {
        write_op_rm_mr(f, op, "mem_reg");
    }

    writeln!(f, "}}").unwrap();
}

fn main() {
    cargo_emit::rerun_if_changed!("{}", Op::path().to_str().unwrap());

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fns.rs");
    let mut f = File::create(&dest_path).unwrap();

    write_ops(&mut f);
}
