use std::path::PathBuf;

use serde::Deserialize;

use crate::{cmd::result::CmdResult, files::{get_files, get_dirs}};

#[derive(Debug, Deserialize, Clone)]
pub struct ProConfig {
    pub package: Package,
    pub profile: Profile
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub enum PackageType {
    #[serde(rename = "exe")]
    #[default]
    Executable,
    #[serde(rename = "lib")]
    Library
}

#[derive(Debug, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)] 
    pub ptype: PackageType
}

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    pub compiler: Option<String>,
    pub src: Vec<String>,
    pub include: Option<Vec<String>>,
    pub libs: Option<Vec<String>>,
    pub lib: Option<Vec<String>>,
    pub precomp: Option<Vec<String>>
}

impl Profile {
    /// Get the source files for this project as compiler arguments.
    pub fn source_args(&self, pwd: &PathBuf) -> CmdResult<Vec<PathBuf>> {
        get_files(pwd, &self.src)
    }

    /// Get the includes for this project as compiler arguments.
    pub fn include_args(&self, pwd: &PathBuf) -> CmdResult<Option<Vec<String>>> {
        let Some(ref inc) = self.include else {
            return Ok(None)
        };

        let dirs = get_dirs(pwd, &inc)?;

        Ok(Some(dirs.iter().map(|i| format!("-I{}", i.to_string_lossy())).collect()))
    }

    /// Get the precompiled files for this project as compiler arguments.
    pub fn precomp_args(&self, pwd: &PathBuf) -> Option<CmdResult<Vec<PathBuf>>> {
        let Some(ref precomp) = self.precomp else {
            return None
        };

        Some(get_files(pwd, &precomp))
    }
}
