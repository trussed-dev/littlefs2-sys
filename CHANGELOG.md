# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Derive `Default` if possible

## [0.3.1] - 2025-02-27

### Fixed

- Fix code generation with `multiversion` feature

## [0.3.0] - 2025-02-26

### Changed

- Update littlefs to [v2.9.3](https://github.com/littlefs-project/littlefs/releases/tag/v2.9.3)
  - Add `multiversion` feature

### Fixed

- Fix build script for platforms where the Rust target is not equal to the clang target ([#16](https://github.com/trussed-dev/littlefs2-sys/pull/16))

## [0.2.0] - 2024-05-28

### Added

- Add `malloc` feature flag. It allows `littlefs` to link to `malloc` and `free` instead of relying on the caller to allocate memory ([#9][])
- Add a `software-intrinsics` feature flag, to disable the use of compiler intrinsics when compiling littlefs ([#12][])

### Changed

- Upgrade `bindgen` to 0.69.4 and limit symbols to those prefixed with `lfs_` and `LFS_` ([#10][])
- Use `core::ffi::*` instead of `cty::*` ([#14][])

[#9]: https://github.com/trussed-dev/littlefs2-sys/pull/10
[#10]: https://github.com/trussed-dev/littlefs2-sys/pull/10
[#12]: https://github.com/trussed-dev/littlefs2-sys/pull/12
[#14]: https://github.com/trussed-dev/littlefs2-sys/pull/14

## [0.1.7] - 2022-01-26

### Fixed

- Fixed compilation issue caused by other crates also using `bindgen` by selecting the `runtime` feature ([#5])

[#5]: https://github.com/trussed-dev/littlefs2-sys/pull/5
[#9]: https://github.com/trussed-dev/littlefs2-sys/pull/9

[Unreleased]: https://github.com/trussed-dev/littlefs2-sys/compare/0.3.1...HEAD
[0.1.7]: https://github.com/trussed-dev/littlefs2-sys/compare/0.1.6...0.1.7
[0.2.0]: https://github.com/trussed-dev/littlefs2-sys/compare/0.1.7...0.2.0
[0.3.0]: https://github.com/trussed-dev/littlefs2-sys/compare/0.2.0...0.3.0
[0.3.1]: https://github.com/trussed-dev/littlefs2-sys/compare/0.3.0...0.3.1
