use std::path::PathBuf;

use crate::{cmd::result::CmdResult, werror};

/// Get all files from a list of file match patterns within the working directory.
pub fn get_files(p: &PathBuf, patterns: &Vec<String>) -> CmdResult<Vec<PathBuf>> {
    let mut files = Vec::<PathBuf>::with_capacity(64);

    // Go through all file patterns and collect the files into a vector.
    for pattern in patterns {
        let path = p.join(pattern).to_string_lossy().to_string();

        // Perform the file glob.
        let paths = match glob::glob(&path) {
            Ok(r) => Ok(r),
            Err(err) => Err(werror!("manifest", "invalid file matching pattern `{}`.\n{}", &pattern, err))
        }?;

        // Check for errors and push file into files buffer.
        let mut looped = false;
        for path in paths {
            let path = match path {
                Ok(r) => Ok(r),
                Err(err) => Err(werror!("manifest", "file glob error for `{}`.\n{}", &pattern, err))
            }?;

            if path.is_file() {
                files.push(path);
            }
            looped = true;
        }

        // Return error if no paths were looped over.
        if looped == false {
            return Err(werror!("manifest", "source file(s) not found `{}`.", &pattern));
        }
    }

    Ok(files)
}

/// Get all directories from a list of file match patterns within the working directory.
pub fn get_dirs(p: &PathBuf, patterns: &Vec<String>) -> CmdResult<Vec<PathBuf>> {
    let mut files = Vec::<PathBuf>::with_capacity(64);

    // Go through all file patterns and collect the files into a vector.
    for pattern in patterns {
        // Ignore wacky patterns.
        let pattern = pattern.trim_end_matches("*").trim_end_matches("**/");
        let path = p.join(pattern);

        if path.exists() == false {
            return Err(werror!("manifest", "include directory not found `{}`.", pattern));
        }

        files.push(path);
    }

    Ok(files)
}
