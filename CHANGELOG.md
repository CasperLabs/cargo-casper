# Changelog

All notable changes to this project will be documented in this file.  The format is based on [Keep a Changelog].

[comment]: <> (Added:      new features)
[comment]: <> (Changed:    changes in existing functionality)
[comment]: <> (Deprecated: soon-to-be removed features)
[comment]: <> (Removed:    now removed features)
[comment]: <> (Fixed:      any bug fixes)
[comment]: <> (Security:   in case of vulnerabilities)


## [2.2.1] - 2024-06-21

### Added
* `--wrapper` option for using `rustc` wrapper to build reproducible wasm.



## [2.2.0] - 2024-02-09

### Changed
* Update dependencies to match casper-node release 1.5.6.
* Restrict pinned nightly Rust version to only smart contract folder of generated project.



## [2.1.1] - 2023-11-27

### Changed
* Update pinned version of Rust toolchain to `nightly-2023-03-25`.
* Update generated test for missing runtime arg.
* Update dependencies to match casper-node release 1.5.5.
* Update generated projects to use `edition = 2021` in their manifests.
* Avoid accommodating for `main` branch in integration tests, as this branch has been permanently deleted in favour of a progression of `release-x.y.z` branches.



## [2.1.0] - 2023-05-12

### Changed
* Update dependencies to match casper-node release 1.5.0.



## [2.0.10] - 2023-05-10

### Changed
* Update dependencies.



## [2.0.9] - 2023-04-28

### Changed
* Update dependencies.



## [2.0.8] - 2023-04-20

### Changed
* Update dependencies.



## [2.0.7] - 2023-03-08

### Changed
* Update pinned version of Rust toolchain to `nightly-2022-08-03` and update dependencies.



## [2.0.6] - 2023-01-04

### Changed
* Update pinned version of Rust toolchain and dependencies.



## [2.0.5] - 2022-05-13

### Changed
* Update dependencies.



## [2.0.4] - 2022-04-06

### Changed
* Update dependencies.



## [2.0.3] - 2022-02-07

### Changed
* Update pinned version of Rust toolchain to `nightly-2022-01-13` and update dependencies.



## [2.0.2] - 2022-01-06

### Changed
* Update dependencies.



## [2.0.1] - 2021-12-17

### Changed
* Update dependencies.



## [2.0.0] - 2021-11-30

### Changed
* Update to use the v2 `casper-engine-test-support` crate in generated projects' tests.
* Move from casper-node monorepo to its own standalone repo.



## [1.4.2] - 2021-11-13

### Removed
* Remove support for generating an ERC-20 contract.



## [1.4.1] - 2021-10-23

No changes.



## [1.4.0] - 2021-10-21 [YANKED]

### Changed
* Support building and testing using stable Rust.



## [1.3.4] - 2021-10-14

No changes.



## [1.3.3] - 2021-09-13

### Added
* Add support for generating an ERC-20 contract.



## [1.3.2] - 2021-08-02

No changes.



## [1.3.1] - 2021-07-25

No changes.



## [1.3.0] - 2021-07-20

### Changed
* Update pinned version of Rust to `nightly-2021-06-17`.



## [1.2.1] - 2021-07-17

No changes.



## [1.2.0] - 2021-05-28

### Changed
* Change to Apache 2.0 license.



## [1.1.1] - 2021-04-19

No changes.



## [1.1.0] - 2021-04-13 [YANKED]

No changes.



## [1.0.1] - 2021-04-08

No changes.



## [1.0.0] - 2021-03-30

### Added
* Initial release of cargo-casper compatible with Casper mainnet.



[Keep a Changelog]: https://keepachangelog.com/en/1.0.0
[Unreleased]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.2.0...dev
[2.2.0]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.1.1...v2.2.0
[2.1.1]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.1.0...v2.1.1
[2.1.0]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.10...v2.1.0
[2.0.10]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.9...v2.0.10
[2.0.9]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.8...v2.0.9
[2.0.8]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.7...v2.0.8
[2.0.7]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.6...v2.0.7
[2.0.6]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.5...v2.0.6
[2.0.5]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.4...v2.0.5
[2.0.4]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.3...v2.0.4
[2.0.3]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.2...v2.0.3
[2.0.2]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/casper-ecosystem/cargo-casper/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.4.2...v2.0.0
[1.4.2]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.4.1...v1.4.2
[1.4.1]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.4.0...v1.4.1
[1.4.0]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.3.3...v1.4.0
[1.3.4]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.3.3...v1.3.4
[1.3.3]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.3.2...v1.3.3
[1.3.2]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.2.1...v1.3.0
[1.2.1]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/casper-ecosystem/cargo-casper/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/casper-ecosystem/cargo-casper/tree/v1.0.0
