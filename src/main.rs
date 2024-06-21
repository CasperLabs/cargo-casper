//! Command line tool for creating a Wasm contract and tests for use on the Casper Platform.

// #![deny(warnings)]

mod common;
mod contract_package;
mod dependency;
mod makefile;
mod tests_package;
mod travis_yml;
mod wrapper;

use std::{
    env,
    path::{Path, PathBuf},
};

use clap::{
    builder::ValueParser, crate_description, crate_name, crate_version, Arg, ArgAction, Command,
};
use once_cell::sync::Lazy;

const USAGE: &str = r"cargo casper [FLAGS] <path>
    cd <path>
    make prepare
    make test";

const ROOT_PATH_ARG_NAME: &str = "path";
const ROOT_PATH_ARG_HELP: &str = "Path to new folder for contract and tests";

const WORKSPACE_PATH_ARG_NAME: &str = "workspace-path";

const GIT_URL_ARG_NAME: &str = "git-url";

const GIT_BRANCH_ARG_NAME: &str = "git-branch";

const WRAPPER_ARG_NAME: &str = "wrapper";
const WRAPPER_ARG_HELP: &str = "Use rustc wrapper to ensure wasm reproducibility";

const FAILURE_EXIT_CODE: i32 = 101;

static ARGS: Lazy<Args> = Lazy::new(Args::new);

/// Can be used (via hidden command line args) to specify a patch section for the casper crates in
/// the generated Cargo.toml files.
#[derive(Debug)]
enum CasperOverrides {
    /// The path to local copy of the casper-node repository.
    WorkspacePath(PathBuf),
    /// The details of an online copy of the casper-node repository.
    GitRepo { url: String, branch: String },
}

#[derive(Debug)]
struct Args {
    root_path: PathBuf,
    casper_overrides: Option<CasperOverrides>,
    use_wrapper: bool,
}

impl Args {
    fn new() -> Self {
        // If run normally, the args passed are 'cargo-casper', '<target dir>'.  However, if run as
        // a cargo subcommand (i.e. cargo casper <target dir>), then cargo injects a new arg:
        // 'cargo-casper', 'casper', '<target dir>'.  We need to filter this extra arg out.
        //
        // This yields the situation where if the binary receives args of 'cargo-casper', 'casper'
        // then it might be a valid call (not a cargo subcommand - the user entered
        // 'cargo-casper casper' meaning to create a target dir called 'casper') or it might be an
        // invalid call (the user entered 'cargo casper' with no target dir specified).  The latter
        // case is assumed as being more likely.
        let filtered_args_iter = env::args().enumerate().filter_map(|(index, value)| {
            if index == 1 && value == "casper" {
                None
            } else {
                Some(value)
            }
        });

        let root_path_arg = Arg::new(ROOT_PATH_ARG_NAME)
            .value_parser(ValueParser::path_buf())
            .required(true)
            .value_name(ROOT_PATH_ARG_NAME)
            .help(ROOT_PATH_ARG_HELP);

        let use_wrapper_arg = Arg::new(WRAPPER_ARG_NAME)
            .long(WRAPPER_ARG_NAME)
            .short('w')
            .help(WRAPPER_ARG_HELP)
            .action(ArgAction::SetTrue);

        let workspace_path_arg = Arg::new(WORKSPACE_PATH_ARG_NAME)
            .long(WORKSPACE_PATH_ARG_NAME)
            .hide(true);

        let git_url_arg = Arg::new(GIT_URL_ARG_NAME)
            .long(GIT_URL_ARG_NAME)
            .hide(true)
            .conflicts_with(WORKSPACE_PATH_ARG_NAME)
            .requires(GIT_BRANCH_ARG_NAME);

        let git_branch_arg = Arg::new(GIT_BRANCH_ARG_NAME)
            .long(GIT_BRANCH_ARG_NAME)
            .hide(true)
            .conflicts_with(WORKSPACE_PATH_ARG_NAME)
            .requires(GIT_URL_ARG_NAME);

        let arg_matches = Command::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .override_usage(USAGE)
            .arg(root_path_arg)
            .arg(use_wrapper_arg)
            .arg(workspace_path_arg)
            .arg(git_url_arg)
            .arg(git_branch_arg)
            .get_matches_from(filtered_args_iter);

        let root_path = arg_matches
            .get_one::<PathBuf>(ROOT_PATH_ARG_NAME)
            .expect("expected path")
            .clone();
        let use_wrapper = arg_matches.get_flag(WRAPPER_ARG_NAME);
        let maybe_workspace_path = arg_matches.get_one::<String>(WORKSPACE_PATH_ARG_NAME);
        let maybe_git_url = arg_matches.get_one::<String>(GIT_URL_ARG_NAME);
        let maybe_git_branch = arg_matches.get_one::<String>(GIT_BRANCH_ARG_NAME);

        let casper_overrides = match (maybe_workspace_path, maybe_git_url, maybe_git_branch) {
            (Some(path), None, None) => Some(CasperOverrides::WorkspacePath(path.into())),
            (None, Some(url), Some(branch)) => Some(CasperOverrides::GitRepo {
                url: url.to_string(),
                branch: branch.to_string(),
            }),
            (None, None, None) => None,
            _ => unreachable!("Clap rules enforce either both or neither git args are present"),
        };

        Args {
            root_path,
            casper_overrides,
            use_wrapper,
        }
    }

    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    pub fn casper_overrides(&self) -> Option<&CasperOverrides> {
        self.casper_overrides.as_ref()
    }
}

fn main() {
    if ARGS.root_path.exists() {
        common::print_error_and_exit(&format!(
            ": destination '{}' already exists",
            ARGS.root_path.display()
        ));
    }

    common::create_dir_all(&ARGS.root_path);
    contract_package::create();
    tests_package::create();
    makefile::create(ARGS.use_wrapper);
    travis_yml::create();
    if ARGS.use_wrapper {
        wrapper::create();
    }
}
