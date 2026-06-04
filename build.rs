use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let name = env::var("CARGO_PKG_NAME").unwrap();
    
    let content = format!(
        r#"pub const VERSION: &str = "{}";
pub const NAME: &str = "{}";
"#,
        version, name
    );
    
    fs::write(&dest_path, content).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
