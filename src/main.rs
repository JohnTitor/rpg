use clap::Clap;
use std::process::Command;

mod run;
mod share;

/// CLI tool for running your code on the Rust Playground.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Yuki Okushi <huyuumi.dev@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Run(Run),
    Share(Share),
}

/// A subcommand for running a snippet on the playground.
/// Open your default browser with passed code.
#[derive(Clap)]
struct Run {
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

#[derive(Clap)]
struct Share {
    /// File name contains code you want to share.
    file_name: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Run(r) => match crate::run::run(&r) {
            Ok(url) => {
                if cfg!(target_os = "windows") {
                    let status = Command::new("rundll32.exe")
                        .args(&["url.dll,FileProtocolHandler", &url])
                        .status()
                        .expect("failed to open browser");
                    assert!(status.success());
                } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
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
                    unimplemented!()
                }
            }
            Err(e) => {
                eprintln!("failed to execute `run` command: {}", e);
                std::process::exit(1);
            }
        }
        SubCommand::Share(s) => match crate::share::share(&s) {
            Ok(id) => {
                println!("Share URL: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist={}", id);
            }
            Err(e) => {
                eprintln!("failed to execute `share` command: {}", e);
                std::process::exit(1);
            }
        }
    }
}
