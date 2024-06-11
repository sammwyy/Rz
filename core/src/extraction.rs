use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use zip::{read::ZipFile, ZipArchive};

use crate::{utils::resolve_relative, RzError, RzSettings};

fn extract_file(file: &mut ZipFile, dest: &PathBuf, _settings: &RzSettings) -> Result<(), RzError> {
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

    // Set the file permissions.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if _settings.unix_permissions.is_some() {
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

    Ok(())
}

fn extract_with_picking(
    mut zip: ZipArchive<File>,
    dest: &PathBuf,
    pick: Vec<String>,
    settings: &RzSettings,
) -> Result<(), RzError> {
    for filename in pick {
        let mut file = match zip.by_name(filename.as_str()) {
            Ok(file) => file,
            Err(..) => {
                return Err(RzError::EntryNotFound(filename));
            }
        };

        extract_file(&mut file, dest, settings)?;
    }

    Ok(())
}

fn extract_all(
    mut zip: ZipArchive<File>,
    dest: &PathBuf,
    settings: &RzSettings,
) -> Result<(), RzError> {
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        extract_file(&mut file, dest, settings)?;
    }

    Ok(())
}

pub fn extract(
    src: PathBuf,
    dest: PathBuf,
    pick: Option<Vec<String>>,
    settings: RzSettings,
) -> Result<(), RzError> {
    // Resolve the paths.
    let src = resolve_relative(src);
    let dest = resolve_relative(dest);

    // Check if the source file exists.
    if !Path::new(&src).exists() {
        return Err(RzError::FileNotFound(src.to_str().unwrap().to_string()));
    }

    // Open the zip file.
    let file = File::open(&src)?;
    let zip = ZipArchive::new(file)?;

    // Check for cherry-picking argument.
    if pick.is_some() {
        extract_with_picking(zip, &dest, pick.unwrap(), &settings)?;
    } else {
        extract_all(zip, &dest, &settings)?;
    }

    Ok(())
}
