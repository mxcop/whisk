use std::{path::Path, fs, ffi::OsStr};

/// Get all files with a specific file extension within a directory recursively.
pub(crate) fn get_all_files(p: &Path, ext: &str) -> std::io::Result<Vec<String>> {
    let mut files = Vec::<String>::with_capacity(64);

    walk_path(&mut files, p, ext)?;

    Ok(files)
}

/// Recursively walk and collect all files in a directory.
pub(crate) fn walk_path(
    files: &mut Vec<String>,
    p: &Path,
    ext: &str,
) -> std::io::Result<()> {
    // Stop walking if path isn't a directory.
    if !p.is_dir() {
        return Ok(())
    }

    let dir = fs::read_dir(p)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        walk_path(files, &path, ext)?;

        let file_ext = path.extension().and_then(OsStr::to_str);
        if let Some(file_ext) = file_ext {
            if file_ext == ext {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}
