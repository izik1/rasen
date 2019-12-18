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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct OpZaxImm {
    name: String,
    args: Vec<String>,
    op8: u8,
    op: u8,
    op16_prefix: u8,
}

impl Opcode for OpZaxImm {
    const FILE_NAME: &'static str = "op_zax_imm.json";
}

fn write_op_zax_imm(f: &mut File, op: OpZaxImm) {
    assert_eq!(&op.args, &["zax".to_owned(), "imm".to_owned()]);
    writeln!(f, r#"    pub fn {name}<Width: WWidth>(&mut self, imm: impl Immediate<Width>) -> io::Result<()> {{
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

fn write_ops(f: &mut File) {
    writeln!(f, "impl<'a, T: io::Write + io::Seek> Assembler<'a, T> {{").unwrap();

    for op in serde_json::from_reader::<_, Vec<_>>(File::open(OpZaxImm::path()).unwrap()).unwrap() {
        write_op_zax_imm(f, op);
    }

    writeln!(f, "}}").unwrap();
}

fn main() {
    cargo_emit::rerun_if_changed!("{}", OpZaxImm::path().to_str().unwrap());

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fns.rs");
    let mut f = File::create(&dest_path).unwrap();

    write_ops(&mut f);
}
