use std::path::{Path, PathBuf};

use rzip_core::{
    utils::{gather_files, resolve_relative},
    Rz, RzError, RzSettings,
};

pub fn append(src: Vec<PathBuf>, dest: PathBuf, settings: RzSettings) -> Result<(), RzError> {
    let mut rz = Rz::new(dest);
    rz.open(true)?;

    let files = gather_files(src);
    rz.append_entries(files?, settings)?;

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

    // Create rz client.
    let mut rz = Rz::new(src);
    rz.open(false)?;

    // Check for cherry-picking argument.
    if pick.is_some() {
        rz.extract_picking(dest, pick.unwrap())?;
    } else {
        rz.extract_all(dest)?;
    }

    Ok(())
}

pub fn list(src: PathBuf, pick: Option<Vec<String>>) -> Result<(), RzError> {
    // Resolve the paths.
    let src = resolve_relative(src);

    // Check if the source file exists.
    if !Path::new(&src).exists() {
        return Err(RzError::FileNotFound(src.to_str().unwrap().to_string()));
    }

    // Create rz client.
    let mut rz = Rz::new(src);
    rz.open(false)?;

    // Check for cherry-picking argument.
    let entries = match pick {
        Some(pick) => rz.list(pick[0].clone())?,
        None => rz.list("".to_string())?,
    };

    for entry in entries {
        println!("{}", entry.path);
    }

    Ok(())
}
