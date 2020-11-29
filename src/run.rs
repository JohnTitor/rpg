//! A module for `run` subcommand.
//!
//! ## Usage
//!
//! ```sh
//! rpg run <file-name>
//!```
//! file-name: A file contains code to run.

use clap::Clap;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::fs::File;
use std::io::prelude::*;

use crate::error::RpgError;
use crate::{validate_opts, Validator};

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

    let code = utf8_percent_encode(&code, NON_ALPHANUMERIC).to_string();
    let url = format!(
        "https://play.rust-lang.org/?version={}&mode={}&edition={}&code={}",
        run.version, run.mode, run.edition, code
    );

    Ok(url)
}
