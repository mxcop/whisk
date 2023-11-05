use std::{path::PathBuf, collections::BTreeMap};

use serde::Deserialize;

use crate::{cmd::result::CmdResult, files::{get_files, get_dirs}};

/// ## Whisk Manifest
/// 
/// Deserializable struct representing the whisk manifest toml.
#[derive(Debug, Deserialize, Clone)]
pub struct WhiskManifest {
    /// Package configuration.
    pub package: Package,

    /// Optional, build targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Targets>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    #[serde(default)] 
    pub lang: PackageLanguage,
    #[serde(rename = "type")]
    #[serde(default)] 
    pub ptype: PackageType,

    #[serde(flatten)]
    pub target: Target
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub enum PackageLanguage {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "c++")]
    #[default]
    CXX
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
pub struct Target {
    pub compiler: Option<String>,
    pub src: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub libs: Option<Vec<String>>,
    pub lib: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Targets {
    #[serde(flatten)]
    pub custom: BTreeMap<String, Target>,
}

impl Package {
    /// Get the programming language defined for this package.
    pub fn get_lang(&self) -> &'static str {
        match self.lang {
            PackageLanguage::C => "c",
            PackageLanguage::CXX => "c++",
        }
    }
}

impl Target {
    /// Get the source files for this target as compiler arguments.
    pub fn source_args(&self, pwd: &PathBuf) -> CmdResult<Vec<PathBuf>> {
        match self.src.as_ref() {
            Some(src) => get_files(pwd, src),
            None => Ok(Vec::new()),
        }
    }

    /// Get the includes for this target as compiler arguments.
    pub fn include_args(&self, pwd: &PathBuf) -> CmdResult<Option<Vec<String>>> {
        let Some(ref inc) = &self.include else {
            return Ok(None)
        };

        let dirs = get_dirs(pwd, &inc)?;

        Ok(Some(dirs.iter().map(|i| format!("-I{}", i.to_string_lossy())).collect()))
    }
}
