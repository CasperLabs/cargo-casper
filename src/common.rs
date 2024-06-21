use std::{fs, path::Path, process, str};

use colour::e_red;
use once_cell::sync::Lazy;

use crate::{dependency::Dependency, CasperOverrides, ARGS, FAILURE_EXIT_CODE};

pub static CL_CONTRACT: Lazy<Dependency> =
    Lazy::new(|| Dependency::new("casper-contract", "4.0.0"));
pub static CL_TYPES: Lazy<Dependency> = Lazy::new(|| Dependency::new("casper-types", "4.0.1"));
pub static CL_ENGINE_TEST_SUPPORT: Lazy<Dependency> =
    Lazy::new(|| Dependency::new("casper-engine-test-support", "7.0.1"));
pub static CL_EXECUTION_ENGINE: Lazy<Dependency> =
    Lazy::new(|| Dependency::new("casper-execution-engine", "7.0.1"));
pub static PATCH_SECTION: Lazy<String> = Lazy::new(|| match ARGS.casper_overrides() {
    Some(CasperOverrides::WorkspacePath(path)) => {
        format!(
            r#"[patch.crates-io]
casper-contract = {{ path = "{0}/smart_contracts/contract" }}
casper-engine-test-support = {{ path = "{0}/execution_engine_testing/test_support" }}
casper-execution-engine = {{ path = "{0}/execution_engine" }}
casper-types = {{ path = "{0}/types" }}
"#,
            path.display()
        )
    }
    Some(CasperOverrides::GitRepo { url, branch }) => {
        format!(
            r#"[patch.crates-io]
casper-contract = {{ git = "{url}", branch = "{branch}" }}
casper-engine-test-support = {{ git = "{url}", branch = "{branch}" }}
casper-execution-engine = {{ git = "{url}", branch = "{branch}" }}
casper-types = {{ git = "{url}", branch = "{branch}" }}
"#,
        )
    }
    None => String::new(),
});

pub fn print_error_and_exit(msg: &str) -> ! {
    e_red!("error");
    eprintln!("{msg}");
    process::exit(FAILURE_EXIT_CODE)
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) {
    if let Err(error) = fs::create_dir_all(path.as_ref()) {
        print_error_and_exit(&format!(
            ": failed to create '{}': {error}",
            path.as_ref().display(),
        ));
    }
}

pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) {
    if let Err(error) = fs::write(path.as_ref(), contents) {
        print_error_and_exit(&format!(
            ": failed to write to '{}': {error}",
            path.as_ref().display(),
        ));
    }
}

#[cfg(test)]
pub mod tests {
    use reqwest::blocking;
    use serde_json::Value;

    use super::*;

    const CRATES_IO_RAW_INDEX_URL_FOR_CASPER_CRATES: &str =
        "https://raw.githubusercontent.com/rust-lang/crates.io-index/master/ca/sp/";
    const CRATES_IO_INDEX_URL_FOR_CASPER_CRATES: &str =
        "https://github.com/rust-lang/crates.io-index/blob/master/ca/sp/";
    const VERSION_FIELD_NAME: &str = "vers";

    /// Checks the version of the package specified by the Cargo.toml at `toml_path` is equal to
    /// the hard-coded one specified in `dep.version()`.

    /// https://crates.io/data-access
    fn check_latest_published_casper_package_version(dep: &Dependency) {
        let url = format!(
            "{}{}",
            CRATES_IO_RAW_INDEX_URL_FOR_CASPER_CRATES,
            dep.name()
        );
        let crate_io_index_contents = blocking::get(url)
            .unwrap_or_else(|error| {
                panic!(
                    "should get index file for {} from GitHub: {error}",
                    dep.name(),
                )
            })
            .text()
            .unwrap_or_else(|error| panic!("should parse index file for {}: {error}", dep.name()));

        let latest_entry: Value = serde_json::from_str(
            crate_io_index_contents
                .lines()
                .last()
                .expect("index file should contain at least one entry"),
        )
        .expect("latest entry from index file should parse as JSON");
        let latest_publish_version = latest_entry[VERSION_FIELD_NAME].as_str().unwrap();

        // If this fails, ensure `dep.version()` is updated to match the value in the Cargo.toml at
        // `toml_url`.
        assert_eq!(
            latest_publish_version,
            dep.version(),
            "\n\nEnsure local version of {dep:?} in common.rs is updated to \
            {latest_publish_version} as defined in last version of \
            {CRATES_IO_INDEX_URL_FOR_CASPER_CRATES}{}\n\n",
            dep.name()
        );
    }

    #[test]
    fn check_cl_contract_version() {
        check_latest_published_casper_package_version(&CL_CONTRACT);
    }

    #[test]
    fn check_cl_types_version() {
        check_latest_published_casper_package_version(&CL_TYPES);
    }

    #[test]
    fn check_cl_engine_test_support_version() {
        check_latest_published_casper_package_version(&CL_ENGINE_TEST_SUPPORT);
    }

    #[test]
    fn check_cl_execution_engine_version() {
        check_latest_published_casper_package_version(&CL_EXECUTION_ENGINE);
    }
}
