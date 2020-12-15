//! A module for `run` subcommand.
//!
//! ## Usage
//!
//! ```sh
//! rpg run <file-name>
//! ```
//! file-name: A file contains code to run.

use clap::Clap;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::error::RpgError;
use crate::{validate_opts, Validator};

/// Execute given code and return the result on the playground.
const RUN_URL: &str = "https://play.rust-lang.org/execute";

/// A subcommand for running a snippet on the playground.
/// Open your default browser with passed code.
#[derive(Clap)]
pub(crate) struct Run {
    /// File name to execute.
    file_name: String,
    #[clap(short, long, default_value = "stable")]
    /// rustc version, panic if not `stable`, `beta`, or `nightly`.
    version: String,
    #[clap(short, long, default_value = "debug")]
    /// Opt level, panic if not `debug` or `release`.
    mode: String,
    #[clap(short, long, default_value = "2018")]
    /// Edition, panic if not `2015` or `2018`.
    edition: String,
    #[clap(short, long)]
    /// Open your default browser with given code, if passed.
    pub open: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RunReq {
    backtrace: bool,
    channel: String,
    code: String,
    crate_type: String,
    edition: String,
    mode: String,
    tests: bool,
}

impl Default for RunReq {
    fn default() -> Self {
        Self {
            backtrace: false,
            channel: "stable".to_string(),
            code: "".to_string(),
            crate_type: "bin".to_string(),
            edition: "2018".to_string(),
            mode: "debug".to_string(),
            tests: false,
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct RunRes {
    success: bool,
    stdout: String,
    stderr: String,
}

impl Validator for Run {
    fn version(&self) -> String {
        self.version.clone()
    }
    fn mode(&self) -> String {
        self.mode.clone()
    }
    fn edition(&self) -> String {
        self.edition.clone()
    }
}

pub(crate) fn run(run: &Run) -> Result<String, RpgError> {
    validate_opts(run);

    let mut file = File::open(&run.file_name)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    if run.open {
        let code = utf8_percent_encode(&code, NON_ALPHANUMERIC).to_string();
        let url = format!(
            "https://play.rust-lang.org/?version={}&mode={}&edition={}&code={}",
            run.version, run.mode, run.edition, code
        );
        Ok(url)
    } else {
        println!("Running code on the playground...\n");
        let req = RunReq {
            code,
            ..Default::default()
        };
        let client = reqwest::blocking::Client::new();
        let res: RunRes = client.post(RUN_URL).json(&req).send()?.json()?;
        let res = format!("{}\n{}", res.stderr, res.stdout);

        Ok(res)
    }
}
