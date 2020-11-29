use std::{fs::File, collections::HashMap};
use std::io::prelude::*;
use serde::Deserialize;
use clap::Clap;

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

pub(crate) fn share(share: &Share) -> std::io::Result<String> {
    let mut file = File::open(&share.file_name)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;
    let mut map = HashMap::new();
    map.insert("code", &code);

    let client = reqwest::blocking::Client::new();
    let res: GistRes = client.post("https://play.rust-lang.org/meta/gist/")
        .json(&map)
        .send()
        .unwrap()
        .json()
        .unwrap();
    Ok(res.id)
}
