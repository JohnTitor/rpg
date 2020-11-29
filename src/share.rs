use clap::Clap;
use serde::Deserialize;
use std::io::prelude::*;
use std::{collections::HashMap, fs::File};

use crate::error::RpgError;

/// Returns Gist URL, ID, and given code itself.
const GIST_GEN_URL: &str = "https://play.rust-lang.org/meta/gist/";

/// A subcommand for generating permanent playground URL.
#[derive(Clap)]
pub struct Share {
    /// File name contains code you want to share.
    file_name: String,
}

#[derive(Deserialize)]
struct GistRes {
    id: String,
}

pub(crate) fn share(share: &Share) -> Result<String, RpgError> {
    let mut file = File::open(&share.file_name)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut req_json = HashMap::new();
    req_json.insert("code", &code);

    let client = reqwest::blocking::Client::new();
    let res: GistRes = client.post(GIST_GEN_URL).json(&req_json).send()?.json()?;

    Ok(res.id)
}
