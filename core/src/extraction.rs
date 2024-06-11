use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use zip::{read::ZipFile, ZipArchive};

use crate::{utils::resolve_relative, RzError};

fn extract_file(file: &mut ZipFile, dest: &PathBuf) -> Result<(), RzError> {
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

fn extract_with_picking(
    mut zip: ZipArchive<File>,
    dest: &PathBuf,
    pick: Vec<String>,
) -> Result<(), RzError> {
    for filename in pick {
        let mut file = match zip.by_name(filename.as_str()) {
            Ok(file) => file,
            Err(..) => {
                return Err(RzError::EntryNotFound(filename));
            }
        };

        extract_file(&mut file, dest)?;
    }

    Ok(())
}

fn extract_all(mut zip: ZipArchive<File>, dest: &PathBuf) -> Result<(), RzError> {
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        extract_file(&mut file, dest)?;
    }

    Ok(())
}

pub fn extract(src: PathBuf, dest: PathBuf, pick: Option<Vec<String>>) -> Result<(), RzError> {
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
        extract_with_picking(zip, &dest, pick.unwrap())?;
    } else {
        extract_all(zip, &dest)?;
    }

    Ok(())
}
