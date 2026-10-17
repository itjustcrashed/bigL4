use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    let linker_dir = manifest_dir.join("link");
    let linker_script_name: &str;

    if target.contains("riscv64gc-unknown-neon-kernel") {
        linker_script_name = "kernel-riscv64.ld";
    } else {
        panic!("unsupported target: {}", target);
    }

    println!("cargo:rustc-link-search=native={}", linker_dir.display());
    println!("cargo:rustc-link-arg=-T{}", linker_script_name);

    let linker_script_path = linker_dir.join(linker_script_name);
    println!("cargo:rerun-if-changed={}", linker_script_path.display());
    println!("cargo:rerun-if-changed=build.rs");
}
