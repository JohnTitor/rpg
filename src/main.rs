use clap::Clap;
use std::process::Command;

mod run;

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
}

/// A subcommand for running a snippet on the playground.
/// Currently, the passed code isn't formatted well.
/// Use rustfmt on the playground if needed.
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

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Run(t) => match crate::run::run(&t) {
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
        },
    }
}
