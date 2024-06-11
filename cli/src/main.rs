use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_utils::{options_from_args, ClapCompressionMethod};
use rzip_core::{RzError, RzSettings};

pub mod clap_utils;

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
        #[clap(
            name = "sources",
            help = "(Paths) Source files or directories",
            required = true
        )]
        src: Vec<PathBuf>,

        #[clap(name = "dest", help = "(Path) Destination file", required = true)]
        dest: PathBuf,
    },

    /**
     * Compress a file or directory.
     */
    #[clap(alias = "c", about = "Compress a file or directory")]
    Compress {
        #[clap(
            name = "sources",
            help = "(Paths) Source files or directories",
            required = true
        )]
        src: Vec<PathBuf>,

        #[clap(name = "dest", help = "(Path) Destination file", required = true)]
        dest: PathBuf,
    },

    /**
     * Extract a file on a directory.
     */
    #[clap(alias = "x", about = "Extract a file on a directory")]
    Extract {
        #[clap(name = "source", help = "(Path) Source file.")]
        src: PathBuf,

        #[clap(name = "dest", help = "(Path) Destination file or directory")]
        dest: PathBuf,

        #[arg(short = 'p', long = "pick", help = "Pick files to extract")]
        pick: Option<Vec<String>>,
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
pub struct Args {
    // Subcommands.
    #[command(subcommand)]
    command: Command,

    // Options.
    #[arg(short = 'l', long = "level", help = "Compression level to use")]
    compression_level: Option<i64>,

    #[arg(short = 'm', long = "method", help = "Compression method to use")]
    method: Option<ClapCompressionMethod>,

    #[arg(short = 'u', long = "unix", help = "Set unix permissions (i.e 755)")]
    unix_permissions: Option<u32>,
}

// Commands implementation.
fn append(src: Vec<PathBuf>, dest: PathBuf, settings: RzSettings) -> Result<(), RzError> {
    rzip_core::append::append(src, dest, settings)
}

fn compress(src: Vec<PathBuf>, dest: PathBuf, settings: RzSettings) -> Result<(), RzError> {
    rzip_core::compression::compress(src, dest, settings)
}

fn extract(
    src: PathBuf,
    dest: PathBuf,
    pick: Option<Vec<String>>,
    settings: RzSettings,
) -> Result<(), RzError> {
    rzip_core::extraction::extract(src, dest, pick, settings)
}

/**
* Main function.
*/
fn main() {
    let args = Args::parse();
    let opts = options_from_args(&args);

    let err = match args.command {
        Command::Append { src, dest } => append(src, dest, opts),
        Command::Compress { src, dest } => compress(src, dest, opts),
        Command::Extract { src, dest, pick } => extract(src, dest, pick, opts),
    };

    if let Err(e) = err {
        eprintln!("{}", e);
    }
}
