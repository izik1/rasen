use std::collections::HashSet;
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
    op8: Option<u8>,
    op: u8,
    rm: Option<u8>,
    mm: Option<u8>,
    min: u8,
    max: u8,
}

impl Op {
    fn mm(&self) -> String {
        if let Some(mm) = self.mm {
            format!("Some({:#02x?})", mm)
        } else {
            "None".to_owned()
        }
    }
}

#[derive(Debug, serde_derive::Deserialize, Clone)]
struct SingleSizeOp {
    name: String,
    op: u8,
    rm: Option<u8>,
    mm: Option<u8>,
}

impl SingleSizeOp {
    fn mm(&self) -> String {
        if let Some(mm) = self.mm {
            format!("Some({:#02x?})", mm)
        } else {
            "None".to_owned()
        }
    }
}

#[derive(Debug, serde_derive::Deserialize, Clone)]
struct VexOp {
    name: String,
    op: u8,
    rm: Option<u8>,
    mm: u8,
    pp: u8,
}

#[derive(Debug, serde_derive::Deserialize)]
struct Ops {
    zax_imm: Vec<Op>,
    rm_imm: Vec<Op>,
    rm_imm8: Vec<Op>,
    rm_sximm8: Vec<SingleSizeOp>,
    reg_rm: Vec<Op>,
    rm_reg: Vec<Op>,
    no_operands: Vec<SingleSizeOp>,
    reg_rm_reg: Vec<VexOp>,
}

impl Opcode for Op {
    const FILE_NAME: &'static str = "ops.json";
}

fn write_op_zax_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {{
        self.op_zax_imm(imm, {op8:#02x?}, {op:#02x?})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op)).unwrap();
}

fn write_op_reg_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R, imm: impl Immediate<Width>) -> io::Result<()> {{
        self.op_reg_imm(reg, imm, {op8:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op), rm=op.rm.unwrap()).unwrap();
}

fn write_op_mem_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_mem_imm<Width: WWidth, M: Memory<Width>>(&mut self, mem: M, imm: impl Immediate<Width>) -> io::Result<()> {{
        self.op_mem_imm(mem, imm, {op8:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op), rm=op.rm.unwrap()).unwrap();
}

fn write_op_reg_imm8(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_reg_imm8<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R, imm: u8) -> io::Result<()> {{
        self.op_reg_imm8(reg, imm, {op8:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op), rm=op.rm.unwrap()).unwrap();
}

fn write_op_mem_imm8(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_mem_imm8<Width: WWidth, M: Memory<Width>>(&mut self, mem: M, imm: u8) -> io::Result<()> {{
        self.op_mem_imm8(mem, imm, {op8:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op), rm=op.rm.unwrap()).unwrap();
}

fn write_op_reg_sximm8(f: &mut File, op: SingleSizeOp) {
    writeln!(f, r#"    pub fn {name}_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(&mut self, reg: R, imm: i8) -> io::Result<()> {{
        self.op_reg_imm8(reg, imm as u8, {op:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, rm=op.rm.unwrap()).unwrap();
}

fn write_op_mem_sximm8(f: &mut File, op: SingleSizeOp) {
    writeln!(f, r#"    pub fn {name}_mem_sximm8<Width: WidthAtLeast16, M: Memory<Width>>(&mut self, mem: M, imm: i8) -> io::Result<()> {{
        self.op_mem_imm8(mem, imm as u8, {op:#02x?}, {op:#02x?}, {rm})
    }}
"#, name=op.name, op=op.op, rm=op.rm.unwrap()).unwrap();
}

fn width_bound(op: &Op) -> String {
    match (op.min > 8, op.max < 64) {
        (true, true) => format!("WidthAtLeast{} + WidthAtMost{}", op.min, op.max),
        (true, false) => format!("WidthAtLeast{}", op.min),
        (false, true) => format!("WidthAtMost{}", op.max),
        (false, false) => "WWidth".to_owned(),
    }
}

fn write_op_mem_reg(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_mem_reg<Width: {width_bound}, R, M>(&mut self, mem: M, reg: R) -> io::Result<()> where R: GeneralRegister<Width>, M: Memory<Width> {{
        self.op_rm_mr(reg, mem, {op8:#02x?}, {op:#02x?}, {mm})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op), width_bound=width_bound(&op), mm=op.mm()).unwrap();
}

fn write_op_reg_mem(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_reg_mem<Width: {width_bound}, R, M>(&mut self, reg: R, mem: M) -> io::Result<()> where R: GeneralRegister<Width>, M: Memory<Width> {{
        self.op_rm_mr(reg, mem, {op8:#02x?}, {op:#02x?}, {mm})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op), width_bound=width_bound(&op), mm=op.mm()).unwrap();
}

fn write_op_reg_reg(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_reg_reg<Width: {width_bound}, R1, R2>(&mut self, reg1: R1, reg2: R2) -> io::Result<()> where R1: GeneralRegister<Width>, R2: GeneralRegister<Width> {{
        self.op_reg_reg(reg1, reg2, {op8:#02x?}, {op:#02x?}, {mm})
    }}
"#, name=op.name, op=op.op, op8=op.op8.unwrap_or(op.op), width_bound=width_bound(&op), mm=op.mm()).unwrap();
}

fn write_op_reg_mem_reg(f: &mut File, op: VexOp) {
    writeln!(f, r#"    pub fn {name}_reg_mem_reg<Width: WidthAtLeast32, R1, M, R2>(&mut self, rd: R1, mem: M, rs: R2) -> io::Result<()>
        where R1: GeneralRegister<Width>, M: Memory<Width>, R2: GeneralRegister<Width>
    {{
        self.op_reg_mem_reg(rd, mem, rs, {mm:#02x?}, {op:#02x?}, {pp:#02x?})
    }}
"#, name=op.name, op=op.op, mm=op.mm, pp=op.pp).unwrap();
}

fn write_op_reg_reg_reg(f: &mut File, op: VexOp) {
    writeln!(f, r#"    pub fn {name}_reg_reg_reg<Width: WidthAtLeast32, RD, RS1, RS2>(&mut self, rd: RD, rs1: RS1, rs2: RS2) -> io::Result<()>
        where RD: GeneralRegister<Width>, RS1: GeneralRegister<Width>, RS2: GeneralRegister<Width>
    {{
        self.op_reg_reg_reg(rd, rs1, rs2, {mm:#02x?}, {op:#02x?}, {pp:#02x?})
    }}
"#, name=op.name, op=op.op, mm=op.mm, pp=op.pp).unwrap();
}

fn write_op_no_operand(f: &mut File, op: SingleSizeOp) {
    assert_eq!(op.rm, None);
    writeln!(
        f,
        r#"    pub fn {name}(&mut self) -> io::Result<()> {{
        self.op_no_operands({op:#02x?}, {mm})
    }}
"#,
        name = op.name,
        op = op.op,
        mm = op.mm()
    )
    .unwrap();
}

#[allow(unused_macros)]
macro_rules! skip_name {
    ($name:literal, $op:ident) => {
        if $op.name == $name {
            continue;
        }
    };
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

    for op in ops.rm_imm8 {
        write_op_reg_imm8(f, op.clone());
        write_op_mem_imm8(f, op);
    }

    for op in ops.rm_sximm8 {
        write_op_reg_sximm8(f, op.clone());
        write_op_mem_sximm8(f, op);
    }

    let mut reg_reg_ops = HashSet::new();

    for op in ops.reg_rm {
        write_op_reg_mem(f, op.clone());
        reg_reg_ops.insert(op.name.clone());
        write_op_reg_reg(f, op);
    }

    for op in ops.rm_reg {
        write_op_mem_reg(f, op.clone());

        // todo: do this anyway, but with a suffix.
        if reg_reg_ops.insert(op.name.clone()) {
            write_op_reg_reg(f, op);
        }
    }

    for op in ops.no_operands {
        write_op_no_operand(f, op);
    }

    for op in ops.reg_rm_reg {
        write_op_reg_mem_reg(f, op.clone());
        write_op_reg_reg_reg(f, op);
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
