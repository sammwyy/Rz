use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use zip::ZipWriter;

use crate::utils::{gather_files, resolve_relative, settings_to_file_options};
use crate::{RzError, RzSettings};

/**
* Compress files to a zip file.
*/
pub fn compress(src: Vec<PathBuf>, dest: PathBuf, settings: RzSettings) -> Result<(), RzError> {
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
