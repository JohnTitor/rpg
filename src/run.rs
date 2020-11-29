use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::fs::File;
use std::io::prelude::*;

use crate::Run;

pub(crate) fn run(run: &Run) -> std::io::Result<String> {
    let mut file = File::open(&run.file_name)?;
    validate_opts(run);
    let mut code = String::new();
    file.read_to_string(&mut code)?;
    let code = utf8_percent_encode(&code, NON_ALPHANUMERIC).to_string();
    let url = format!(
        "https://play.rust-lang.org/?version={}&mode={}&edition={}&code={}",
        run.version, run.mode, run.edition, code
    );
    Ok(url)
}

/// Validate options of `run` command.
fn validate_opts(run: &Run) {
    // FIXME: More elegant handling.
    if !(run.version == "stable" || run.version == "beta" || run.version == "nightly") {
        panic!("`version` should be one of `stable`, `beta`, or `nightly`");
    }
    if !(run.mode == "debug" || run.mode == "release") {
        panic!("`mode` should be one of `debug` or `release`");
    }
    if !(run.edition == "2015" || run.edition == "2018") {
        panic!("`edition` should be one of `2015` or `2018`");
    }
}
