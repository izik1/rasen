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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone)]
struct Op {
    name: String,
    op8: u8,
    op: u8,
    rm: String,
    op16_prefix: u8,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct WidthAtLeast16Op {
    name: String,
    op: u8,
    rm: String,
    op16_prefix: u8,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct Ops {
    zax_imm: Vec<Op>,
    rm_imm: Vec<Op>,
    rm_sximm8: Vec<WidthAtLeast16Op>,
}

impl Opcode for Op {
    const FILE_NAME: &'static str = "ops.json";
}

fn write_op_zax_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_zax_imm<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {{
        if Width::IS_W16 {{
            self.write_byte({op_16_prefix:#02x?})?;
        }}

        let mut rex_byte = 0_u8;

        if Width::HAS_REXW {{
            rex_byte |= 0b0100_1000;
        }}

        let opcode: u8 = if Width::IS_W8 {{
            {op8:#02x?}
        }} else {{
            {op:#02x?}
        }};

        if rex_byte != 0x00 {{
            self.write_byte(rex_byte)?;
        }}

        self.write_byte(opcode)?;

        self.write_immediate(imm.as_writable())
    }}
"#, name=op.name, op_16_prefix=op.op16_prefix, op=op.op, op8=op.op8).unwrap();
}

fn write_op_reg_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_reg_imm<Width: WWidth, R: GeneralRegister<Width>>(&mut self, reg: R, imm: impl Immediate<Width>) -> io::Result<()> {{
        if Width::IS_W16 {{
            self.write_byte({op_16_prefix:#02x?})?;
        }}

        let mut rex_byte = 0_u8;
        if reg.needs_rexb() {{
            rex_byte |= 0b0100_0001;
        }}

        if Width::HAS_REXW {{
            rex_byte |= 0b0100_1000;
        }}

        // SPL, BPL, SIL, DIL are the registers that this matters for.
        if Width::IS_W8 && reg.value() >= 4 {{
            // SPL, BPL, SIL, DIL
            rex_byte |= 0b0100_0000;
        }}

        let opcode: u8 = if Width::IS_W8 {{
            {op8:#02x?}
        }} else {{
            {op:#02x?}
        }};

        if rex_byte != 0x00 {{
            self.write_byte(rex_byte)?;
        }}

        self.write_byte(opcode)?;

        const MOD_RM_REG: u8 = 0b1100_0000;
        let mod_rm_opcode = {rm} << 3;
        let mod_rm = MOD_RM_REG | mod_rm_opcode | (reg.value() % 8);

        self.write_byte(mod_rm)?;

        self.write_immediate(imm.as_writable())
    }}
"#, name=op.name, op_16_prefix=op.op16_prefix, op=op.op, op8=op.op8, rm=op.rm).unwrap();
}

fn write_op_mem_imm(f: &mut File, op: Op) {
    writeln!(f, r#"    pub fn {name}_mem_imm<Width: WWidth>(&mut self, mem: Mem, imm: impl Immediate<Width>) -> io::Result<()> {{
        if Width::IS_W16 {{
            self.write_byte({op_16_prefix:#02x?})?;
        }}

        let mut rex_byte = mem.rex_byte();

        if Width::HAS_REXW {{
            rex_byte |= 0b0100_1000;
        }}

        if rex_byte != 0x00 {{
            self.write_byte(rex_byte)?;
        }}

        let opcode: u8 = if Width::IS_W8 {{
            {op8:#02x?}
        }} else {{
            {op:#02x?}
        }};

        self.write_byte(opcode)?;

        let (mod_rm, sib, displacement) = mem.encoded();

        self.write_mod_rm(mod_rm.with_op({rm}))?;

        if let Some(sib) = sib {{
            self.write_sib(sib)?;
        }}

        if let Some(displacement) = displacement {{
            self.write_displacement(displacement)?;
        }}

        self.write_immediate(imm.as_writable())
    }}
"#, name=op.name, op_16_prefix=op.op16_prefix, op=op.op, op8=op.op8, rm=op.rm).unwrap();
}

fn write_op_reg_sximm8(f: &mut File, op: WidthAtLeast16Op) {
    writeln!(f, r#"    pub fn {name}_reg_sximm8<Width: WidthAtLeast16, R: GeneralRegister<Width>>(&mut self, reg: R, imm: i8) -> io::Result<()> {{
        if Width::IS_W16 {{
            self.write_byte({op_16_prefix:#02x?})?;
        }}

        let mut rex_byte = 0_u8;
        if reg.needs_rexb() {{
            rex_byte |= 0b0100_0001;
        }}

        if Width::HAS_REXW {{
            rex_byte |= 0b0100_1000;
        }}

        let opcode: u8 = {op:#02x?};

        if rex_byte != 0x00 {{
            self.write_byte(rex_byte)?;
        }}

        self.write_byte(opcode)?;

        const MOD_RM_REG: u8 = 0b1100_0000;
        let mod_rm_opcode = {rm} << 3;
        let mod_rm = MOD_RM_REG | mod_rm_opcode | (reg.value() % 8);

        self.write_byte(mod_rm)?;

        self.write_byte(imm as u8)
    }}
"#, name=op.name, op_16_prefix=op.op16_prefix, op=op.op, rm=op.rm).unwrap();
}

fn write_ops(f: &mut File) {
    writeln!(f, "impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {{").unwrap();

    let ops: Ops = serde_json::from_reader(File::open(Op::path()).unwrap()).unwrap();

    for op in ops.zax_imm
    {
        write_op_zax_imm(f, op);
    }

    for op in ops.rm_imm {
        write_op_reg_imm(f, op.clone());
        write_op_mem_imm(f, op);
    }

    for op in ops.rm_sximm8 {
        write_op_reg_sximm8(f, op);
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
