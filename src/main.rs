use clap::Clap;
use std::process::Command;

mod error;
mod run;
mod share;

use crate::run::Run;
use crate::share::Share;

/// CLI tool for running your code on the Rust Playground.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Yuki Okushi <huyuumi.dev@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Subcommands for RPG.
#[derive(Clap)]
enum SubCommand {
    Run(Run),
    Share(Share),
}

/// A helper trait to validate some options.
trait Validator {
    fn version(&self) -> String;
    fn mode(&self) -> String;
    fn edition(&self) -> String;
}

/// Validate options and cause a panic if it's unexpected.
pub(crate) fn validate_opts<T>(cmd: &T)
where
    T: Validator,
{
    // FIXME: More elegant handling.
    let (version, mode, edition) = (cmd.version(), cmd.mode(), cmd.edition());
    if !(version == "stable" || version == "beta" || version == "nightly") {
        panic!("`version` should be one of `stable`, `beta`, or `nightly`");
    }
    if !(mode == "debug" || mode == "release") {
        panic!("`mode` should be one of `debug` or `release`");
    }
    if !(edition == "2015" || edition == "2018") {
        panic!("`edition` should be one of `2015` or `2018`");
    }
}

fn open_browser(url: &str) {
    if cfg!(target_os = "windows") {
        // ref. https://stackoverflow.com/a/49115945
        let status = Command::new("rundll32.exe")
            .args(&["url.dll,FileProtocolHandler", url])
            .status()
            .expect("failed to open browser");
        assert!(status.success());
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        // ref. https://dwheeler.com/essays/open-files-urls.html
        #[cfg(target_os = "macos")]
        let cmd = "open";
        #[cfg(target_os = "linux")]
        let cmd = "xdg-open";

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            let status = Command::new(cmd)
                .arg(url)
                .status()
                .expect("failed to open browser");
            assert!(status.success());
        }
    } else {
        // Ignore other platforms for now.
        unimplemented!()
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Run(r) => match crate::run::run(&r) {
            Ok(url) => {
                open_browser(&url);
            }
            Err(e) => {
                eprintln!("failed to execute `run` command: {}", e);
                std::process::exit(1);
            }
        },
        SubCommand::Share(s) => match crate::share::share(&s) {
            Ok(url) => {
                // FIXME: Should be able to specify version, mode, and edition.
                println!("Share URL: {}", url);
            }
            Err(e) => {
                eprintln!("failed to execute `share` command: {}", e);
                std::process::exit(1);
            }
        },
    }
}
