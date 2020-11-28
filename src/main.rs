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
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Run(t) => {
            println!("file name is {}.", t.file_name);
            match crate::run::run(&t.file_name) {
                Ok(url) => {
                    if cfg!(target_os = "windows") {
                        let status = Command::new("rundll32.exe")
                            .args(&["url.dll,FileProtocolHandler", &url])
                            .status()
                            .expect("failed to execute process");
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
                                .expect("failed to execute process");
                            assert!(status.success());
                        }
                    } else {
                        unimplemented!()
                    }
                }
                Err(_) => {
                    eprintln!("Failed to execute `run` command; check your file and contents.");
                    std::process::exit(1);
                }
            }
        }
    }
}
