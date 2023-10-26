use std::path::PathBuf;

use crate::{cmd::result::CmdResult, werror};

/// Get all files from a list of file match patterns within the working directory.
pub(crate) fn get_files(p: &PathBuf, patterns: &Vec<String>) -> CmdResult<Vec<PathBuf>> {
    let mut files = Vec::<PathBuf>::with_capacity(64);

    // Go through all file patterns and collect the files into a vector.
    for pattern in patterns {
        // Perform the file glob.
        let paths = match glob::glob(p.join(pattern).to_str().unwrap()) {
            Ok(r) => Ok(r),
            Err(err) => Err(werror!("Invalid file matching pattern `{}`.\n{}", &pattern, err))
        }?;

        // Check for errors and push file into files buffer.
        for path in paths {
            let path = match path {
                Ok(r) => Ok(r),
                Err(err) => Err(werror!("File glob error for `{}`.\n{}", &pattern, err))
            }?;

            if path.is_file() {
                files.push(path);
            }
        }
    }

    Ok(files)
}

/// Get all directories from a list of file match patterns within the working directory.
pub(crate) fn get_dirs(patterns: &Vec<String>) -> CmdResult<Vec<PathBuf>> {
    let mut files = Vec::<PathBuf>::with_capacity(64);

    // Go through all file patterns and collect the files into a vector.
    for pattern in patterns {
        // Ignore wacky patterns.
        let pattern = pattern.trim_end_matches("*").trim_end_matches("**/");
        files.push(PathBuf::from(pattern));
    }

    Ok(files)
}
