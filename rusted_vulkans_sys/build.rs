extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(windows)]
fn get_library_paths() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let llvm_path = PathBuf::from(env::var("LLVM_PATH").unwrap());
    let vulkan_path = PathBuf::from(env::var("VULKAN_PATH").unwrap());

    let mut llvm_include = llvm_path.clone();
    llvm_include.push("include");

    let mut llvm_lib = llvm_path.clone();
    llvm_lib.push("lib");

    let mut vulkan_include = vulkan_path.clone();
    vulkan_include.push("Include");

    let mut vulkan_lib = vulkan_path.clone();
    vulkan_lib.push("Lib");

    (llvm_include, llvm_lib, vulkan_include, vulkan_lib)
}

fn main() {
    #[cfg(windows)] {
        let (llvm_include, llvm_lib, vulkan_include, vulkan_lib) =
            get_library_paths();
        println!("cargo:rustc-link-search=native={}", llvm_lib.into_os_string().to_str().unwrap());
        println!("cargo:rustc-link-search=native={}", vulkan_lib.into_os_string().to_str().unwrap());

        println!("cargo:include={}", llvm_include.into_os_string().to_str().unwrap());
        println!("cargo:include={}", vulkan_include.into_os_string().to_str().unwrap());
        println!("cargo:rustc-link-lib=vulkan");
    }

    #[cfg(unix)] {
        println!("cargo:rustc-link-lib=vulkan");
    }

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h");

    builder = builder
            .disable_untagged_union()
            .derive_debug(false);

    #[cfg(windows)] {
        let (_, _, vulkan_include, _) = get_library_paths();

        builder = builder
            // .rust_target(bindgen::RustTarget::Stable_1_33)
            //.disable_untagged_union()
            //.derive_debug(false)
            .clang_arg(format!("-I{}", vulkan_include.into_os_string().to_str().unwrap()))
//            .raw_line("use winapi::shared::minwindef::*;")
        ;
    }
    
    let bindings = builder.generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}