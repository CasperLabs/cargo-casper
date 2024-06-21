//! Consts and functions used to generate the files comprising the "contract" package when running
//! the tool.

use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::{
    common::{self, CL_CONTRACT, CL_TYPES, PATCH_SECTION},
    ARGS,
};

const PACKAGE_NAME: &str = "contract";
static CONTRACT_PACKAGE_ROOT: Lazy<PathBuf> =
    Lazy::new(|| ARGS.root_path().join(PACKAGE_NAME.replace('-', "_")));
static CARGO_TOML: Lazy<PathBuf> = Lazy::new(|| CONTRACT_PACKAGE_ROOT.join("Cargo.toml"));
static MAIN_RS: Lazy<PathBuf> = Lazy::new(|| CONTRACT_PACKAGE_ROOT.join("src/main.rs"));
static CONFIG_TOML: Lazy<PathBuf> = Lazy::new(|| CONTRACT_PACKAGE_ROOT.join(".cargo/config.toml"));
static RUST_TOOLCHAIN: Lazy<PathBuf> = Lazy::new(|| CONTRACT_PACKAGE_ROOT.join("rust-toolchain"));

static CONTRACT_DEPENDENCIES: Lazy<String> = Lazy::new(|| {
    format!(
        "{}{}",
        CL_CONTRACT.display_with_features(true, &[]),
        CL_TYPES.display_with_features(true, &[]),
    )
});

const CONFIG_TOML_CONTENTS: &str = r#"[build]
target = "wasm32-unknown-unknown"
"#;

static CARGO_TOML_CONTENTS: Lazy<String> = Lazy::new(|| {
    format!(
        r#"[package]
name = "{PACKAGE_NAME}"
version = "0.1.0"
edition = "2021"

[dependencies]
{}
[[bin]]
name = "{}"
path = "src/main.rs"
bench = false
doctest = false
test = false

[profile.release]
codegen-units = 1
lto = true

{}"#,
        &*CONTRACT_DEPENDENCIES,
        PACKAGE_NAME.replace('-', "_"),
        &*PATCH_SECTION
    )
});

const MAIN_RS_CONTENTS: &str = include_str!("../resources/main.rs.in");

const RUST_TOOLCHAIN_CONTENTS: &str = include_str!("../resources/rust-toolchain.in");

pub fn create() {
    // Create "<PACKAGE_NAME>/src" folder and write "main.rs" inside.
    let src_folder = MAIN_RS.parent().expect("should have parent");
    common::create_dir_all(src_folder);

    common::write_file(&*MAIN_RS, MAIN_RS_CONTENTS);

    // Create "<PACKAGE_NAME>/.cargo" folder and write "config.toml" inside.
    let config_folder = CONFIG_TOML.parent().expect("should have parent");
    common::create_dir_all(config_folder);
    common::write_file(&*CONFIG_TOML, CONFIG_TOML_CONTENTS);

    // Write "<PACKAGE_NAME>/Cargo.toml".
    common::write_file(&*CARGO_TOML, &*CARGO_TOML_CONTENTS);

    // Write "<PACKAGE_NAME>/rust-toolchain".
    common::write_file(&*RUST_TOOLCHAIN, RUST_TOOLCHAIN_CONTENTS);
}

#[cfg(test)]
mod tests {
    use std::env;

    use reqwest::blocking;

    use super::RUST_TOOLCHAIN_CONTENTS;

    const TEST_BRANCH_NAME: &str = "dev";
    const CRON_JOB_BRANCH_NAME_ENV_VAR: &str = "BRANCH_SELECTOR";
    const PR_TARGET_BRANCH_NAME_ENV_VAR: &str = "GITHUB_BASE_REF";
    const CI_BRANCH_NAME_ENV_VAR: &str = "GITHUB_REF_NAME";
    const CASPER_NODE_TOOLCHAIN_URL: &str =
        "https://raw.githubusercontent.com/casper-network/casper-node/dev/smart_contracts/rust-toolchain";

    /// Checks that the pinned version of Rust is that of the `dev` branch of casper-node.
    ///
    /// If none of `BRANCH_SELECTOR`, `GITHUB_BASE_REF` and `GITHUB_REF_NAME` are set, or they're
    /// all set to empty strings, the test is an auto-pass.
    ///
    /// For testing locally, you can manually run e.g.
    /// ```
    /// BRANCH_SELECTOR=dev cargo t
    /// ```
    #[test]
    fn check_toolchain_version_on_dev() {
        if let Ok(true) = env::var(CRON_JOB_BRANCH_NAME_ENV_VAR)
            .or_else(|_| env::var(PR_TARGET_BRANCH_NAME_ENV_VAR))
            .or_else(|_| env::var(CI_BRANCH_NAME_ENV_VAR))
            .map(|env_var| env_var == TEST_BRANCH_NAME)
        {
        } else {
            println!(
                "skipping 'check_toolchain_version_on_dev' as none of \
                {CRON_JOB_BRANCH_NAME_ENV_VAR}, {PR_TARGET_BRANCH_NAME_ENV_VAR} and \
                {CI_BRANCH_NAME_ENV_VAR} are set to {TEST_BRANCH_NAME}",
            );
            return;
        }

        let expected_toolchain_value = blocking::get(CASPER_NODE_TOOLCHAIN_URL)
            .expect("should get rust-toolchain from GitHub")
            .text()
            .expect("should parse rust-toolchain");

        // If this fails, ensure there's not a mismatch between ../resources/rust-toolchain.in and
        // https://github.com/casper-network/casper-node/blob/dev/smart_contracts/rust-toolchain.
        assert_eq!(&*expected_toolchain_value, RUST_TOOLCHAIN_CONTENTS);
    }
}
