use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use zip::ZipArchive;

use crate::{utils::resolve_relative, RzError, RzSettings};

pub fn extract(src: PathBuf, dest: PathBuf, _settings: RzSettings) -> Result<(), RzError> {
    // Resolve the paths.
    let src = resolve_relative(src);
    let dest = resolve_relative(dest);

    // Check if the source file exists.
    if !Path::new(&src).exists() {
        return Err(RzError::FileNotFound(src.to_str().unwrap().to_string()));
    }

    // Open the zip file.
    let file = File::open(&src)?;
    let mut zip = ZipArchive::new(file)?;

    // Extract the files from the zip file.
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => dest.join(path),
            None => continue,
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
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Set the file permissions.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if settings.unix_permissions.is_some() {
                fs::set_permissions(
                    &outpath,
                    fs::Permissions::from_mode(_settings.unix_permissions.unwrap()),
                )
                .unwrap();
            } else {
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    }

    Ok(())
}
