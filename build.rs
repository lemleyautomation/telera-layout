extern crate bindgen;
extern crate cc;

// Compiles the bundled clay C source. Windows builds go through `clay.cpp` (C++20)
// and everything else through `clay.c` (C99), with an extra `__aarch64__` define for
// aarch64.
//
// The Rust bindings in `src/bindings/clay.rs` are a checked-in file, NOT regenerated
// on every build. Regeneration is opt-in via `TELERA_REGEN_BINDINGS=1` and is meant to
// be run on Linux: clay marks its enums `__attribute__((packed))` (1 byte), which the
// C++ build honors through `enum : uint8_t`. libclang parsing `clay.h` while targeting
// the `*-windows-msvc` triple silently ignores the packed attribute and emits 4-byte
// (`#[repr(i32)]`) enums, so a Windows regeneration produces bindings whose
// `Clay_RenderCommand` layout does not match the compiled library - reading the render
// command array then dereferences a misread pointer and the process dies with
// STATUS_ACCESS_VIOLATION. The Linux-generated bindings (`#[repr(u8)]` enums, fixed
// width integer fields) match the compiled library on every platform.
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    println!("cargo:rerun-if-changed=src/bindings/clay.h");
    println!("cargo:rerun-if-changed=src/bindings/clay.c");
    println!("cargo:rerun-if-changed=src/bindings/clay.cpp");
    println!("cargo:rerun-if-env-changed=TELERA_REGEN_BINDINGS");

    let regen = std::env::var_os("TELERA_REGEN_BINDINGS").is_some();

    if target_os == "windows" {
        cc::Build::new()
            .file("src/bindings/clay.cpp")
            .warnings(false)
            .std("c++20")
            .compile("clay");

        if regen {
            println!(
                "cargo:warning=TELERA_REGEN_BINDINGS on Windows: libclang mis-sizes clay's \
                 packed enums here, producing clay.rs bindings that crash at runtime. \
                 Regenerate on Linux and commit that file instead."
            );
            bindgen::Builder::default()
                .header("src/bindings/clay.h")
                .rustified_enum(".*")
                .derive_debug(true)
                .derive_default(true)
                .generate()
                .expect("Couldn't generate bindings!")
                .write_to_file(std::path::PathBuf::from("src/bindings/clay.rs"))
                .expect("Couldn't write bindings!");

            std::fs::write("bindgenlog.txt", target_arch.clone()).unwrap();
        }
    } else {
        if target_arch == "aarch64" {
            cc::Build::new()
                .std("c99")
                .define("__aarch64__", None)
                .file("src/bindings/clay.c")
                .warnings(false)
                .compile("clay");
        } else {
            cc::Build::new()
                .file("src/bindings/clay.c")
                .warnings(false)
                .compile("clay");
        }

        if regen {
            bindgen::Builder::default()
                .layout_tests(false)
                .clang_macro_fallback()
                .header("src/bindings/clay.h")
                .blocklist_file(".*stdlib.*")
                .blocklist_file(".*pthread.*")
                .blocklist_file(".*glibc.*")
                .blocklist_file(".*pthread_rwlock.*")
                .rustified_enum(".*")
                .derive_debug(true)
                .derive_default(true)
                .generate()
                .expect("Couldn't generate bindings!")
                .write_to_file(std::path::PathBuf::from("src/bindings/clay.rs"))
                .expect("Couldn't write bindings!");
        }
    }
}
