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

Upstream release: [v2.2.0][upstream-release]

[littlefs]: https://github.com/ARMmbed/littlefs
[littlefs2]: https://github.com/nickray/littlefs2
[upstream-release]: https://github.com/ARMmbed/littlefs/releases/tag/v2.2.0

#### License

<sup>littlefs is licensed under [BSD-3-Clause][bsd-3-clause], as are these bindings.</sup>
<br>
<sub>The file `string.c` is licensed under GPL-2.0.<br>
Permissively licensed replacement implementation welcome!</sub>

[bsd-3-clause]: https://github.com/ARMmbed/littlefs/blob/master/LICENSE.md
