use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};
use thiserror::Error;
use zip::result::ZipError;

use zip::{ZipArchive, ZipWriter};

use crate::{
    internals::{extract_file, settings_to_file_options},
    utils::OsFileEntry,
};

mod internals;

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

    #[error("Rzip instance file is not ready yet (open the file first)")]
    RZNotReady,
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

/**
 * Rzip entry.
 */
pub struct RzEntry {
    pub path: String,
    pub is_dir: bool,
}

/**
 * Rzip main struct.
 *
 * This struct is used to interact with the zip file.
 */
pub struct Rz {
    file_path: PathBuf,
    zip: Option<ZipArchive<File>>,
    writer: Option<ZipWriter<File>>,
}

impl Rz {
    /**
     * Create a new Rz instance.
     */
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            zip: None,
            writer: None,
        }
    }

    fn ensure_ready(&self) -> Result<(), RzError> {
        if self.zip.is_none() {
            return Err(RzError::RZNotReady);
        }

        Ok(())
    }

    /**
     * Append a file to the zip.
     */
    pub fn append(&mut self, entry: OsFileEntry, settings: RzSettings) -> Result<(), RzError> {
        self.ensure_ready()?;

        let options = settings_to_file_options(&settings);
        let mut writer = self.writer.as_mut().unwrap();

        if entry.is_dir {
            writer.add_directory(entry.zip_path, options)?;
            return Ok(());
        }

        writer.start_file(entry.zip_path, options)?;
        let mut f = File::open(entry.file_path)?;
        let _ = std::io::copy(&mut f, &mut writer)?;

        Ok(())
    }

    /**
     * Append multiple files to the zip.
     */
    pub fn append_entries(
        &mut self,
        entries: Vec<OsFileEntry>,
        settings: RzSettings,
    ) -> Result<(), RzError> {
        self.ensure_ready()?;

        let options = settings_to_file_options(&settings);
        let mut writer = self.writer.as_mut().unwrap();

        for entry in entries {
            if entry.is_dir {
                writer.add_directory(entry.zip_path, options)?;
                continue;
            }

            writer.start_file(entry.zip_path, options)?;
            let mut f = File::open(entry.file_path)?;
            let _ = std::io::copy(&mut f, &mut writer)?;
        }

        Ok(())
    }

    /**
     * Extract a file from the zip.
     */
    pub fn extract_picking(&mut self, dest: PathBuf, pick: Vec<String>) -> Result<(), RzError> {
        self.ensure_ready()?;

        let zip = self.zip.as_mut().unwrap();
        for filename in pick {
            let mut file = match zip.by_name(filename.as_str()) {
                Ok(file) => file,
                Err(..) => {
                    return Err(RzError::EntryNotFound(filename));
                }
            };

            extract_file(&mut file, &dest)?;
        }

        Ok(())
    }

    /**
     * Extract all files from the zip.
     */
    pub fn extract_all(&mut self, dest: PathBuf) -> Result<(), RzError> {
        self.ensure_ready()?;

        let zip = self.zip.as_mut().unwrap();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            extract_file(&mut file, &dest)?;
        }

        Ok(())
    }

    pub fn list(&mut self, path: String) -> Result<Vec<RzEntry>, RzError> {
        self.ensure_ready()?;

        let zip = self.zip.as_mut().unwrap();
        let mut entries = Vec::new();

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            let file_path = file.name();
            if file_path.starts_with(path.as_str()) {
                entries.push(RzEntry {
                    path: file_path.to_string(),
                    is_dir: file.is_dir(),
                });
            }
        }

        Ok(entries)
    }

    /**
     * Open the zip file.
     */
    pub fn open(&mut self, write: bool) -> Result<(), RzError> {
        let src: &PathBuf = &self.file_path;
        if !Path::new(src).exists() {
            return Err(RzError::FileNotFound(src.to_str().unwrap().to_string()));
        }

        let file = File::open(src)?;
        let zip = ZipArchive::new(file)?;
        self.zip = Some(zip);

        if write {
            let zip_file = OpenOptions::new().read(true).write(true).open(src)?;
            let writer = ZipWriter::new_append(zip_file)?;
            self.writer = Some(writer);
        }

        Ok(())
    }

    /**
     * Close the zip file.
     */
    pub fn close(mut self) -> Result<(), RzError> {
        if self.writer.is_some() {
            self.writer.unwrap().finish()?;
            self.writer = None;
        }

        self.zip = None;
        Ok(())
    }
}
