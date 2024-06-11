use std::path::{Path, PathBuf};

use anyhow::Context;
use walkdir::{DirEntry, WalkDir};

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

#[derive(Clone, Debug)]
pub struct OsFileEntry {
    pub file_path: PathBuf,
    pub zip_path: String,
    pub is_dir: bool,
}

pub fn normalize_entry_name(name: &str) -> String {
    // Replace all backslashes with forward slashes.
    let dash_fix = name.replace("\\", "/");

    // Remove (.) from path.
    let dot_fix = dash_fix.replace("/./", "/");

    return dot_fix;
}

/**
* Gather files from a list of descriptors.
*/
pub fn gather_files(descriptors: Vec<PathBuf>) -> Result<Vec<OsFileEntry>, RzError> {
    let mut files = Vec::new();

    for descriptor in descriptors {
        let file = resolve_relative(descriptor);
        let parent = file.parent().unwrap();
        let name = file.file_name().unwrap().to_str().unwrap().to_string();
        let zip_path = normalize_entry_name(&name);

        if file.is_file() {
            files.push(OsFileEntry {
                file_path: file.to_path_buf(),
                zip_path,
                is_dir: false,
            });
        } else {
            let it = dir_iter(&file.to_path_buf()).unwrap();
            for entry in it {
                let path = entry.path();
                let name = path.strip_prefix(&parent).unwrap();
                let path_as_str = name
                    .to_str()
                    .map(str::to_owned)
                    .with_context(|| format!("{name:?} Is a Non UTF-8 Path"))?;
                let zip_path = normalize_entry_name(&path_as_str);

                files.push(OsFileEntry {
                    file_path: path.to_path_buf(),
                    zip_path,
                    is_dir: path.is_dir(),
                });
            }
        }
    }

    Ok(files)
}

/**
 * Resolve a relative path to an absolute path.
 */
pub fn resolve_relative(relative: PathBuf) -> PathBuf {
    let path = Path::new(&relative);

    if path.is_relative() {
        let current_dir = std::env::current_dir().unwrap();
        let abs_path = current_dir.join(path);
        abs_path
    } else {
        relative
    }
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
