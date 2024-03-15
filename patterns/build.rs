//! Generate rust files for all pomsky expressions in `../expressions`

use pomsky::options::CompileOptions;
use pomsky::Expr;
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

// <manifest.rs
// pub const PATTERNS: &'static [(&'static str, &'static str); 2] = &[
//     ("aws::access_key_id", r#"\<(?:A3T[A-Z0-9]|AKIA|AGPA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}\>"#),
//     ("aws::secret_access_key", r#"\<[0-9a-zA-Z+/]{40}\>"#),
// ];

// <lib.rs
// pub mod manifest;
// pub fn patterns() -> Vec<(&'static str, &'static str)> { manifest::PATTERNS }

fn main() -> io::Result<()> {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out = std::env::var("OUT_DIR").unwrap();

    let expressions_path = Path::new(&manifest).join("src").join("expressions");
    println!(
        "cargo:rerun-if-changed={}",
        expressions_path.to_string_lossy()
    );

    for (key, value) in std::env::vars() {
        eprintln!("{} {}", key, value);
    }

    let mut dst = fs::File::create(Path::new(&out).join("patterns.rs"))?;
    let pomsky_options = CompileOptions {
        flavor: pomsky::options::RegexFlavor::Rust,
        ..Default::default()
    };

    dst.write_all(b"pub const PATTERNS: &'static [(&'static str, &'static str)] = &[\n")?;

    for cat_ent in fs::read_dir(expressions_path)? {
        let cat_ent = cat_ent?;

        for expr_ent in fs::read_dir(cat_ent.path())? {
            let expr_ent = expr_ent?;

            let expr = fs::read_to_string(expr_ent.path())?;

            let pattern = match Expr::parse_and_compile(expr.as_str(), pomsky_options) {
                (Some(compiled), _warnings, _tests) => compiled,
                _ => panic!("failed to parse and compile"),
            };

            dst.write_all(b"(\"")?;
            dst.write_all(cat_ent.file_name().as_encoded_bytes())?;
            dst.write_all(b"::")?;
            dst.write_all(
                expr_ent
                    .file_name()
                    .as_os_str()
                    .as_bytes()
                    .strip_suffix(b".pomsky")
                    .unwrap(),
            )?;
            dst.write_all(b"\", r#\"")?;
            dst.write_all(pattern.as_bytes())?;
            dst.write_all(b"\"#),\n")?;
        }
    }

    dst.write_all(b"];")?;

    Ok(())
}
