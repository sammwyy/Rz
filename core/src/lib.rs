use thiserror::Error;
use zip::result::ZipError;

pub mod append;
pub mod compression;
pub mod extraction;
pub mod utils;

/**
 * Error type for the rzip-core library.
 */
#[derive(Debug, Error)]
pub enum RzError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Entry not found: {0}")]
    EntryNotFound(String),

    #[error("Bad name: {0}")]
    BadName(String),

    #[error("Zip error: {0}")]
    ZipError(#[from] ZipError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("General error: {0}")]
    GeneralError(#[from] anyhow::Error),
}

/**
 * Compression methods.
 */
#[derive(Debug, Clone, Copy)]
pub enum CompressionMethod {
    Stored,
    Deflated,
    DeflatedZlib,
    DeflatedZlibNg,
    Bzip2,
    Zstd,
}

/**
 * Rzip settings.
 */
pub struct RzSettings {
    pub method: Option<CompressionMethod>,
    pub unix_permissions: Option<u32>,
    pub compression_level: Option<i64>,
}
