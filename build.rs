use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .flag("-std=c11")
        .flag("-DLFS_NO_MALLOC")
        .flag("-DLFS_NO_ASSERT")
        .flag("-DLFS_NO_DEBUG")
        .flag("-DLFS_NO_WARN")
        .flag("-DLFS_NO_ERROR")
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c")
        .file("string.c")
        .compile("lfs-sys");

    let bindings = bindgen::Builder::default()
        .header("littlefs/lfs.h")
        .use_core()
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
