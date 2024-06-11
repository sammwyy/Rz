use clap::ValueEnum;
use rzip_core::{CompressionMethod, RzSettings};

use crate::ops::WriteOpFlags;

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
pub fn options_from_write_ops(flags: WriteOpFlags) -> RzSettings {
    let method: Option<CompressionMethod> = if flags.method.is_some() {
        Some(flags.method.clone().unwrap().into())
    } else {
        None
    };

    let compression_level = flags.compression_level;
    let unix_permissions = flags.unix_permissions;

    RzSettings {
        compression_level,
        method,
        unix_permissions,
    }
}
