extern crate bindgen;
extern crate cc;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    //std::fs::write("bindgenlog.txt", target_arch).unwrap();


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