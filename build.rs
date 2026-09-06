extern crate bindgen;
extern crate cc;

// Compiles the bundled clay C source and regenerates `src/bindings/clay.rs` from
// `clay.h`. Windows builds go through `clay.cpp` (C++20) and everything else through
// `clay.c` (C99), with an extra `__aarch64__` define for aarch64. `rustified_enum(".*")`
// turns clay's enums into Rust enums; clay marks them `__attribute__((packed))`, which
// bindgen honors by emitting `#[repr(u8)]`.
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    println!("cargo:rerun-if-changed=src/bindings/clay.h");

    if target_os == "windows" {
        cc::Build::new()
            .file("src/bindings/clay.cpp")
            .warnings(false)
            .std("c++20")
            .compile("clay");

        bindgen::Builder::default()
            .header("src/bindings/clay.h")
            .rustified_enum(".*" )
            .derive_debug(true )
            .derive_default(true)
            .generate()
            .expect("Couldn't generate bindings!")
            .write_to_file(std::path::PathBuf::from("src/bindings/clay.rs")).expect("Couldn't write bindings!");

        std::fs::write("bindgenlog.txt", target_arch.clone()).unwrap();
    } else {
        if target_arch == "aarch64" {
            cc::Build::new()
                .std("c99")
                .define("__aarch64__", None)
                .file("src/bindings/clay.c")
                .warnings(false)
                .compile("clay");
        }
        else {
            cc::Build::new()
                .file("src/bindings/clay.c")
                .warnings(false)
                .compile("clay");
        }
    
        bindgen::Builder::default()
            .layout_tests(false)
            .clang_macro_fallback()
            .header("src/bindings/clay.h")
            .blocklist_file(".*stdlib.*")
            .blocklist_file(".*pthread.*")
            .blocklist_file(".*glibc.*")
            .blocklist_file(".*pthread_rwlock.*")
            .rustified_enum(".*" )
            .derive_debug(true )
            .derive_default(true)
            .generate()
            .expect("Couldn't generate bindings!")
            .write_to_file(std::path::PathBuf::from("src/bindings/clay.rs")).expect("Couldn't write bindings!");
    }
}