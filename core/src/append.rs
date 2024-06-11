use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

use zip::ZipWriter;

use crate::{
    utils::{gather_files, resolve_relative, settings_to_file_options},
    RzError, RzSettings,
};

/**
 * Append files to a zip file.
 */
pub fn append(src: Vec<PathBuf>, dest: PathBuf, settings: RzSettings) -> Result<(), RzError> {
    // Convert the settings to file options.
    let options = settings_to_file_options(&settings);
    // Gather the files to append.
    let files = gather_files(src)?;

    let dest = resolve_relative(dest);
    let dest_path = Path::new(&dest);

    // Open the zip file for appending.
    let zip_file = OpenOptions::new().read(true).write(true).open(dest_path)?;
    let mut zip = ZipWriter::new_append(zip_file)?;

    // Append the files to the zip file.
    for entry in files {
        if entry.is_dir {
            zip.add_directory(entry.zip_path, options)?;
            continue;
        }

        zip.start_file(entry.zip_path, options)?;
        let mut f = File::open(entry.file_path)?;
        let _ = std::io::copy(&mut f, &mut zip)?;
    }

    zip.finish()?;
    Ok(())
}
