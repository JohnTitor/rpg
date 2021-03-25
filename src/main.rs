const HELP: &str = "\
rpg-cli - CLI tool for the Rust Playground

USAGE:
    rpg <SUBCOMMAND> <OPTIONS> <FILENAME>

FLAGS:
    -h, --help       Prints help information

SUBCOMMANDS:
    run         A subcommand for running a snippet on the playground.
                Open your default browser with passed code
    share       A subcommand for generating permanent playground URL

OPTIONS:
    -f, --filename  A file name to be passed to the playground, this must be set
    --version       rustc version, panic if not `stable`, `beta`, or `nightly`
    --mode          Opt level, panic if not `debug` or `release`
    --edition       Edition, panic if not `2015` or `2018`
    --open          Open your default browser with given code (this is only available
                    on the `run` command)

GitHub repo: <https://github.com/JohnTitor/rpg>
";

use std::{path::PathBuf, process::Command};

mod error;
mod run;
mod share;

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
        eprintln!("Error: `version` must be one of `stable`, `beta`, or `nightly`");
        std::process::exit(1);
    }
    if !(mode == "debug" || mode == "release") {
        eprintln!("Error: `mode` must be one of `debug` or `release`");
        std::process::exit(1);
    }
    if !(edition == "2015" || edition == "2018") {
        eprintln!("Error: `edition` must be one of `2015` or `2018`");
        std::process::exit(1);
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
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match args.subcmd {
        Some(v) if v == "run" => {
            let run = crate::run::Run {
                file_name: args.file_name.into_os_string(),
                version: args.version.unwrap_or_else(|| "stable".to_string()),
                mode: args.mode.unwrap_or_else(|| "debug".to_string()),
                edition: args.edition.unwrap_or_else(|| "2018".to_string()),
                open: args.open,
            };
            match crate::run::run(&run) {
                Ok(result) => {
                    if run.open {
                        open_browser(&result);
                    } else {
                        println!("{}", result);
                    }
                }
                Err(e) => {
                    eprintln!("Error: failed to execute `run` command: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(v) if v == "share" => {
            let share = crate::share::Share {
                file_name: args.file_name.into_os_string(),
                version: args.version.unwrap_or_else(|| "stable".to_string()),
                mode: args.mode.unwrap_or_else(|| "debug".to_string()),
                edition: args.edition.unwrap_or_else(|| "2018".to_string()),
            };
            match crate::share::share(&share) {
                Ok(url) => {
                    println!("Share URL: {}", url);
                }
                Err(e) => {
                    eprintln!("Error: failed to execute `share` command: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: subcommand must be `run` or `share`");
            std::process::exit(1);
        }
    };
}

#[derive(Debug)]
struct Args {
    subcmd: Option<String>,
    file_name: PathBuf,
    version: Option<String>,
    mode: Option<String>,
    edition: Option<String>,
    open: bool,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = Args {
        subcmd: pargs.subcommand()?,
        file_name: pargs.value_from_os_str(["-f", "--filename"], parse_path)?,
        version: pargs.opt_value_from_str(["-v", "--version"])?,
        mode: pargs.opt_value_from_str(["-m", "--mode"])?,
        edition: pargs.opt_value_from_str(["-e", "--edition"])?,
        open: pargs.contains("--open"),
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}
