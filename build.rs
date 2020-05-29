use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = cc::Build::new();
    let target = env::var("TARGET").unwrap_or("".to_string());
    let builder = builder
        .flag("-std=c11")
        .flag("-DLFS_NO_MALLOC")
        .flag("-DLFS_NO_DEBUG")
        .flag("-DLFS_NO_WARN")
        .flag("-DLFS_NO_ERROR")
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c")
        .file("string.c")
    ;

    #[cfg(not(feature = "assertions"))]
    let builder = builder.flag("-DLFS_NO_ASSERT");

    #[cfg(feature = "trace")]
    let builder = builder.flag("-DLFS_YES_TRACE");

    builder.compile("lfs-sys");

    let bindings = bindgen::Builder::default()
        .header("littlefs/lfs.h")
        .clang_arg(format!("--target={}", target))
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
