use std::path::PathBuf;

use clap::{Parser, Subcommand};
use ops::{ReadOpFlags, ReadOpTarget, WriteOpFlags, WriteOpTarget};
use rzip_core::RzError;
use utils::options_from_write_ops;

mod handler;
mod ops;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/**
 * Subcommands for the CLI.
*/
#[derive(Debug, Subcommand)]
enum Command {
    /**
     * Append a file or directory to a zip file.
     */
    #[clap(alias = "a", about = "Append a file or directory to a zip file")]
    Append {
        #[clap(flatten)]
        flags: WriteOpFlags,

        #[clap(flatten)]
        target: WriteOpTarget,
    },

    /**
     * Compress a file or directory.
     */
    #[clap(alias = "c", about = "Compress a file or directory")]
    Compress {
        #[clap(flatten)]
        flags: WriteOpFlags,

        #[clap(flatten)]
        target: WriteOpTarget,
    },

    /**
     * Extract a file on a directory.
     */
    #[clap(alias = "x", about = "Extract a file on a directory")]
    Extract {
        #[clap(flatten)]
        flags: ReadOpFlags,

        #[clap(flatten)]
        target: ReadOpTarget,

        #[clap(name = "dest", help = "(Path) Destination directory", required = true)]
        dest: PathBuf,
    },
}

/**
 * CLI arguments.
 */
#[derive(Debug, Parser)]
#[command(
    name = "rz",
    version = VERSION,
    author = "Sammwy",
    about = "Rusty de/compression tool.",
    long_about = "A simple CLI tool to compress and decompress files",
)]
pub struct CLI {
    // Subcommands.
    #[command(subcommand)]
    command: Command,
}

// Commands implementation.
fn cmd_append(src: Vec<PathBuf>, dest: PathBuf, flags: WriteOpFlags) -> Result<(), RzError> {
    let settings = options_from_write_ops(flags);
    handler::append(src, dest, settings)
}

fn compress(src: Vec<PathBuf>, dest: PathBuf, flags: WriteOpFlags) -> Result<(), RzError> {
    let settings = options_from_write_ops(flags);
    rzip_core::utils::compress_to_file(src, dest, settings)
}

fn extract(src: PathBuf, dest: PathBuf, pick: Option<Vec<String>>) -> Result<(), RzError> {
    handler::extract(src, dest, pick)
}

/**
* Main function.
*/
fn main() {
    let args = CLI::parse();

    let err = match args.command {
        Command::Append { flags, target } => cmd_append(target.src, target.dest, flags),
        Command::Compress { flags, target } => compress(target.src, target.dest, flags),
        Command::Extract {
            flags,
            target,
            dest,
        } => extract(target.src, dest, flags.pick),
    };

    if let Err(e) = err {
        eprintln!("{}", e);
    }
}
