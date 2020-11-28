use clap::Clap;

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
        }
    }
}
