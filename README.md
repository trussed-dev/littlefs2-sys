<h1 align="center">littlefs2-sys</h1>
<div align="center">
 <strong>
   Low-level bindings to littlefs
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/littlefs2-sys">
    <img src="https://img.shields.io/crates/v/littlefs2-sys.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/littlefs2-sys">
    <img src="https://img.shields.io/crates/d/littlefs2-sys.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- API docs -->
  <a href="https://docs.rs/littlefs2-sys">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="main branch API docs" />
  </a>
</div>

## What is this?

Low-level bindings to the [littlefs][littlefs] microcontroller filesystem.

You probably want the high-level, idiomatic Rust bindings: [littlefs2][littlefs2]

Upstream release: [v2.9.3][upstream-release]

[littlefs]: https://github.com/ARMmbed/littlefs
[littlefs2]: https://github.com/nickray/littlefs2
[upstream-release]: https://github.com/ARMmbed/littlefs/releases/tag/v2.9.3

## Custom littlefs configuration

littlefs reads its system configuration (logging, assertions, memory allocation,
CRC, ...) from `lfs_util.h`. The `UNSAFE_LITTLEFS_CONFIG` environment variable
can be set to the path of a header that replaces it, which is passed to the C
compiler as `LFS_CONFIG`:

```sh
UNSAFE_LITTLEFS_CONFIG=/path/to/lfs_config.h cargo build
```

It can also be set in `.cargo/config.toml`:

```toml
[env]
UNSAFE_LITTLEFS_CONFIG = { value = "include/lfs_config.h", relative = true }
```

This is unsafe: the header must provide everything `lfs_util.h` provides, and
it can change the behavior of littlefs in ways the Rust bindings cannot check.
See littlefs's [`lfs_util.h`][lfs-config] for what the header needs to define.

[lfs-config]: https://github.com/littlefs-project/littlefs/blob/master/lfs_util.h

#### License

littlefs is licensed under [BSD-3-Clause][bsd-3-clause], as are these bindings.

[bsd-3-clause]: https://github.com/ARMmbed/littlefs/blob/master/LICENSE.md
