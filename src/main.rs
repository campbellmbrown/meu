mod scan;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "meu")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan files for a regex pattern
    #[clap(group(
        clap::ArgGroup::new("target")
            .required(true)
            .args(&["dir", "file"])
    ))]
    Scan {
        /// Directory to search. Mutually exclusive with --file
        #[clap(short, long, group = "target")]
        dir: Option<String>,

        /// File to search. Mutually exclusive with --dir
        #[clap(short, long, group = "target")]
        file: Option<String>,

        /// Regex pattern to search for
        #[clap(short, long)]
        pattern: String,

        /// File glob pattern. Can only be used with --dir
        #[clap(
            short,
            long,
            requires = "dir",
            conflicts_with = "file",
            default_value = "*"
        )]
        glob: String,

        /// Stop on first match in any file
        #[clap(short = 'F', long)]
        first_match: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            dir,
            file,
            pattern,
            glob,
            first_match,
        } => {
            let target = if let Some(file) = file {
                scan::ScanTarget::File { path: file }
            } else if let Some(dir) = dir {
                scan::ScanTarget::Dir {
                    dir,
                    file_glob: glob,
                }
            } else {
                panic!("Either --dir or --file must be provided");
            };

            let args = scan::ScanArgs {
                target,
                pattern,
                stop_on_first_match: first_match,
            };

            scan::run(&args)?;
        }
    }

    Ok(())
}
