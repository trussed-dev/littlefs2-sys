use std::env;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

fn patch_lfs_h(src: &str, dest: &str) {
    let input = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(src)
        .unwrap();
    let reader = BufReader::new(input);

    let output = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest)
        .unwrap();
    let mut writer = BufWriter::new(output);

    for line in reader.lines().map(Result::unwrap) {
        if line != "#include \"lfs_util.h\"" {
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
        }
    }

    writer.flush().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = cc::Build::new();
    let target = env::var("TARGET")?;
    let builder = builder
        .flag("-std=c11")
        .flag("-DLFS_NO_MALLOC")
        .flag("-DLFS_NO_DEBUG")
        .flag("-DLFS_NO_WARN")
        .flag("-DLFS_NO_ERROR")
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c")
        .file("string.c");

    #[cfg(not(feature = "assertions"))]
    let builder = builder.flag("-DLFS_NO_ASSERT");

    #[cfg(feature = "trace")]
    let builder = builder.flag("-DLFS_YES_TRACE");

    builder.compile("lfs-sys");

    let mut lfs_h_out = env::var("OUT_DIR").unwrap();
    lfs_h_out.push_str("/lfs.h");
    // lfs_util.h pulls in various system headers. Some of them use u128 types
    // which bindgen can't handle correctly due to Rust not having stable ABI
    // for u128 type. This results in massive amount of compilation warnings.
    // As a workaround we remove include of `lfs_util.h` file, which have some
    // utilities useful for C programs but not Rust.
    patch_lfs_h("littlefs/lfs.h", &lfs_h_out);

    let bindings = bindgen::Builder::default()
        .header(&lfs_h_out)
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

    Ok(())
}
