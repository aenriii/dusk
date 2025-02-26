use std::process::Command;
use std::env;

fn main() {
    println!("cargo::rerun-if-changed=src/asm/bootloader.s");

    let out_dir = env::var("OUT_DIR").unwrap();


    Command::new("nasm")
        .args(&["-felf32", "src/asm/bootloader.s", "-o"])
        .arg(&format!("{}/bootloader.o", out_dir))
        .status()
        .unwrap();

        println!("cargo::rustc-link-arg={}/bootloader.o", out_dir);
        println!("cargo::rustc-link-arg=-Tkernel.ld");
        println!("cargo::rustc-link-arg=--build-id=none");
}
