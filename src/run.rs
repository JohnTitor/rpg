// 1. get file name
// 2. get file content
// 3. generate URL
// 4. Pass it

use std::fs::File;
use std::io::prelude::*;

pub(crate) fn run(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut url =
        String::from("https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=");
    file.read_to_string(&mut url)?;
    Ok(url)
}
