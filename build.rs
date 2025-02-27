use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = cc::Build::new();
    let builder = builder
        .flag("-std=c99")
        .flag("-DLFS_NO_DEBUG")
        .flag("-DLFS_NO_WARN")
        .flag("-DLFS_NO_ERROR")
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c")
        .file("string.c");

    #[cfg(feature = "software-intrinsics")]
    let builder = builder.flag("-DLFS_NO_INTRINSICS");

    #[cfg(not(feature = "assertions"))]
    let builder = builder.flag("-DLFS_NO_ASSERT");

    #[cfg(feature = "trace")]
    let builder = builder.flag("-DLFS_YES_TRACE");

    #[cfg(not(feature = "malloc"))]
    builder.flag("-DLFS_NO_MALLOC");

    #[cfg(feature = "multiversion")]
    let builder = builder.flag("-DLFS_MULTIVERSION");

    builder.compile("lfs-sys");

    // Patch lfs.h to remove the lfs_util import because clang fails to locate the
    // libraries for the custom target (especially string.h)
    // Compilation before that succeeds because it's using gcc,
    // which comes as a distribution with these utils.
    // Turns out lfs_utils is not used in lfs.h, and clang properly finds stdint.h and stdbool,
    // but not string.h
    let lfs_h = std::fs::read_to_string("littlefs/lfs.h").expect("Reading lfs.h succeeds");
    println!("cargo::rerun-if-changed=littlefs/lfs.h");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_lfs_h = out_path.join("lfs.h");
    std::fs::write(
        &out_lfs_h,
        lfs_h.replace(
            r##"#include "lfs_util.h""##,
            "#include <stdint.h>\n#include <stdbool.h>",
        ),
    )
    .expect("Failed to write lfs.h");

    let bindgen = bindgen::Builder::default()
        .header(out_lfs_h.into_os_string().into_string().unwrap())
        .clang_arg("-std=c99")
        .clang_arg("-DLFS_NO_DEBUG")
        .clang_arg("-DLFS_NO_WARN")
        .clang_arg("-DLFS_NO_ERROR");

    #[cfg(feature = "multiversion")]
    let bindgen = bindgen.clang_arg("-DLFS_MULTIVERSION");

    let bindings = bindgen
        .derive_default(true)
        .use_core()
        .allowlist_item("lfs_.*")
        .allowlist_item("LFS_.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
