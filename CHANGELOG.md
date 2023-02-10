# Changelog: clf

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`

### Changed
* refactored `Makefile`

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`


## [0.1.6] (2023-01-28)
### Added
* `.github/workflows/test.yml`
* test status badges into `README.tpl`

### Fixed
* Makefile: rustc version `1.66.0` to `1.66.1`
* compile error on windows: cl : Command line warning D9002 : ignoring unknown option '-static'
* `LICENSE` files

## [0.1.5] (2023-01-10)
### Added
* rust-version = "1.56.0" into Cargo.toml
* `all-test-version` target into Makefile
* badges into README.tpl

### Changed
* reformat `CHANGELOG.md`

## [0.1.4] (2022-06-13)
### Changed
* changes to edition 2021

## [0.1.3] (2021-11-14)
### Added
* more documents

### Changed
* clean source codes

## [0.1.1] (2021-06-16)
### Fixed
* armv7-unknown-linux-musleabihf: can not compile: not found `__clear_cache()`
* suppress this error by void function

## [0.1.0] (2021-06-16)
* first commit

[Unreleased]: https://github.com/aki-akaguma/clf/compare/v0.1.6..HEAD
[0.1.6]: https://github.com/aki-akaguma/clf/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/clf/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/clf/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/clf/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/clf/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/clf/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/clf/releases/tag/v0.1.0
