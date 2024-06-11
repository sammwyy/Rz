use clap::ValueEnum;
use rzip_core::{CompressionMethod, RzSettings};

use crate::Args;

/**
 * Compression methods for the CLI.
 *
 * This is a value enum that maps the CLI compression methods to the core compression methods.
 */
#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum ClapCompressionMethod {
    Stored,
    Deflated,
    DeflatedZlib,
    DeflatedZlibNg,
    Bzip2,
    Zstd,
}

impl From<ClapCompressionMethod> for CompressionMethod {
    fn from(method: ClapCompressionMethod) -> Self {
        match method {
            ClapCompressionMethod::Stored => CompressionMethod::Stored,
            ClapCompressionMethod::Deflated => CompressionMethod::Deflated,
            ClapCompressionMethod::DeflatedZlib => CompressionMethod::DeflatedZlib,
            ClapCompressionMethod::DeflatedZlibNg => CompressionMethod::DeflatedZlibNg,
            ClapCompressionMethod::Bzip2 => CompressionMethod::Bzip2,
            ClapCompressionMethod::Zstd => CompressionMethod::Zstd,
        }
    }
}

/**
 * Get the rzip settings from the CLI arguments.
 */
pub fn options_from_args(args: &Args) -> RzSettings {
    let method: Option<CompressionMethod> = if args.method.is_some() {
        Some(args.method.clone().unwrap().into())
    } else {
        None
    };

    let unix_permissions = args.unix_permissions;
    let compression_level = args.compression_level;

    RzSettings {
        method,
        unix_permissions,
        compression_level,
    }
}
