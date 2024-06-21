# `cargo-casper`

[![Crates.io](https://img.shields.io/crates/v/cargo-casper)](https://crates.io/crates/cargo-casper)
[![License](https://img.shields.io/badge/license-Apache-blue)](LICENSE)

A command line tool for creating a Wasm smart contract and tests for use on the Casper network.

---

## Installation

`cargo casper` is a Cargo subcommand which can be installed via `cargo install`:

```
cargo install cargo-casper
```

To install from the latest `dev` branch:

```
git clone https://github.com/casper-ecosystem/cargo-casper
cargo install cargo-casper --path=cargo-casper
```

## Usage

To create a folder "my_project" containing a basic example contract and a separate test crate for the contract:

```
cargo casper my_project
```

This creates the following files:

```
my_project/
├── contract
│   ├── .cargo
│   │   └── config.toml
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── Makefile
├── rust-toolchain
├── tests
│   ├── Cargo.toml
│   └── src
│       └── integration_tests.rs
└── .travis.yml
```

### Building the contract

To build the contract, the correct version of Rust must be installed along with the Wasm target:

```
cd my_project
make prepare
```

The contract can now be built using:

```
make build-contract
```

and will be built to `my_project/contract/target/wasm32-unknown-unknown/release/contract.wasm`.

### Testing the contract

Running the test will automatically build the contract in release mode, copy it to the "tests/wasm" folder, then build
and run the test:

```
make test
```

### Reproducibility

Currently, [cargo](https://github.com/rust-lang/cargo/issues/8140) does not provide cross-platform reproducibility for binary files, including WebAssembly. To work around the issue, we provide `rustc` wrapper, which can be enabled using `--wrapper` option.

```
cargo casper my_project --wrapper
```


## License

Licensed under the [Apache License Version 2.0](LICENSE).
