use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use walkdir::{DirEntry, WalkDir};
use zip::read::ZipFile;

use crate::{CompressionMethod, RzError, RzSettings};

/**
 * Get the zip compression method from the Rzip compression method.
 */
pub fn get_zip_method(method: CompressionMethod) -> zip::CompressionMethod {
    match method {
        CompressionMethod::Stored => zip::CompressionMethod::Stored,
        CompressionMethod::Deflated => zip::CompressionMethod::Deflated,
        CompressionMethod::DeflatedZlib => zip::CompressionMethod::Deflated,
        CompressionMethod::DeflatedZlibNg => zip::CompressionMethod::Deflated,
        CompressionMethod::Bzip2 => zip::CompressionMethod::Bzip2,
        CompressionMethod::Zstd => zip::CompressionMethod::Zstd,
    }
}

/**
 * Create a directory iterator.
 */
pub fn dir_iter(dir: &PathBuf) -> Result<impl Iterator<Item = DirEntry>, RzError> {
    if !Path::new(dir).exists() {
        return Err(RzError::FileNotFound(dir.to_str().unwrap().to_string()));
    }

    let walkdir = WalkDir::new(dir);
    let walk_it = walkdir.into_iter();
    let it = walk_it.filter_map(|e| e.ok());
    Ok(it)
}

/**
 * Convert the Rzip settings to zip file options.
 */
pub fn settings_to_file_options(settings: &RzSettings) -> zip::write::FileOptions<'static, ()> {
    let mut options =
        zip::write::FileOptions::default().compression_level(settings.compression_level);

    if let Some(method) = settings.method {
        options = options.compression_method(get_zip_method(method));
    }

    if let Some(permissions) = settings.unix_permissions {
        options = options.unix_permissions(permissions);
    }

    options
}

/**
 * Extract a file from a zip file.
 */
pub fn extract_file(file: &mut ZipFile, dest: &PathBuf) -> Result<(), RzError> {
    let outpath = match file.enclosed_name() {
        Some(path) => dest.join(path),
        None => return Ok(()),
    };

    if file.is_dir() {
        fs::create_dir_all(&outpath)?;
    } else {
        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(&p)?;
            }
        }

        let mut outfile = File::create(&outpath)?;
        std::io::copy(file, &mut outfile)?;
    }

    Ok(())
}
