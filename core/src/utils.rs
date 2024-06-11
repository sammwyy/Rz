use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use zip::ZipWriter;

use crate::{
    internals::{dir_iter, settings_to_file_options},
    RzError, RzSettings,
};

/**
 * Represents a file entry in the OS.
 */
#[derive(Clone, Debug)]
pub struct OsFileEntry {
    pub file_path: PathBuf,
    pub zip_path: String,
    pub is_dir: bool,
}

/**
* Normalize an entry name.
*/
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
* Compress files to a zip file.
*/
pub fn compress_to_file(
    src: Vec<PathBuf>,
    dest: PathBuf,
    settings: RzSettings,
) -> Result<(), RzError> {
    // Convert the settings to file options.
    let options = settings_to_file_options(&settings);
    // Gather the files to compress.
    let files = gather_files(src)?;

    let dest = resolve_relative(dest);
    let dest_path = Path::new(&dest);
    let dest_file = File::create(dest_path).unwrap();

    // Create the zip file.
    let mut zip = ZipWriter::new(dest_file);
    let mut buffer = Vec::new();

    // Compress the files to the zip file.
    for entry in files {
        if entry.is_dir {
            zip.add_directory(entry.zip_path, options)?;
            continue;
        }

        zip.start_file(entry.zip_path, options)?;

        let mut f = File::open(entry.file_path)?;
        f.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;
        buffer.clear();
    }

    zip.finish()?;
    Ok(())
}
