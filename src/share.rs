//! A module for `share` subcommand.
//!
//! ## Usage
//!
//! ```sh
//! rpg share <file-name>
//! ```
//! file-name: A file contains code to share.

use clap::Clap;
use serde::Deserialize;
use std::io::prelude::*;
use std::{collections::HashMap, fs::File};

use crate::error::RpgError;
use crate::{validate_opts, Validator};

/// Returns Gist URL, ID, and given code itself.
const GIST_GEN_URL: &str = "https://play.rust-lang.org/meta/gist/";

/// A subcommand for generating permanent playground URL.
#[derive(Clap)]
pub(crate) struct Share {
    /// File name contains code you want to share.
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

impl Validator for Share {
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

#[derive(Deserialize)]
struct GistRes {
    id: String,
}

pub(crate) fn share(share: &Share) -> Result<String, RpgError> {
    validate_opts(share);

    let mut file = File::open(&share.file_name)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut req_json = HashMap::new();
    req_json.insert("code", &code);

    let client = reqwest::blocking::Client::new();
    let res: GistRes = client.post(GIST_GEN_URL).json(&req_json).send()?.json()?;

    let url = format!(
        "https://play.rust-lang.org/?version={}&mode={}&edition={}&gist={}",
        share.version, share.mode, share.edition, res.id
    );

    Ok(url)
}
