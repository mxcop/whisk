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
    pub target: Option<Targets>,
    pub profile: Option<Profiles>
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
    /// Compiler to use for this target.
    pub compiler: Option<String>,
    /// List of source file globs.
    pub src: Option<Vec<String>>,
    /// List of include directories.
    pub include: Option<Vec<String>>,
    /// List of library directories.
    pub libs: Option<Vec<String>>,
    /// List of libraries.
    pub lib: Option<Vec<String>>,
    /// How to link the target.
    pub link: Option<LinkType>
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub enum LinkType {
    #[serde(rename = "dynamic")]
    #[default]
    Dynamic,
    #[serde(rename = "static")]
    Static
}

impl ToString for LinkType {
    fn to_string(&self) -> String {
        match self {
            LinkType::Dynamic => "-dynamic",
            LinkType::Static => "-static"
        }.to_owned()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Targets {
    #[serde(flatten)]
    pub custom: BTreeMap<String, Target>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    #[serde(rename = "opt-level")]
    pub opt_level: Option<OptimizeLevel>,
    pub debug: Option<DebugLevel>
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Profiles {
    pub release: Option<Profile>,
    pub dev: Option<Profile>,

    #[serde(flatten)]
    pub custom: BTreeMap<String, Profile>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum DebugLevel {
    #[serde(rename = "none", alias = "0")]
    None,
    /// Produces minimal information, enough for making backtraces 
    /// in parts of the program that you don’t plan to debug. 
    /// This includes descriptions of functions and external variables, 
    /// and line number tables, but no information about local variables. 
    #[serde(rename = "min", alias = "minimal", alias = "1")]
    Minimal,
    /// Produces extra information, such as all the macro definitions present in the program. 
    /// Some debuggers support macro expansion when you use -g3.
    #[serde(rename = "full", alias = "extra", alias = "3")]
    Full
}

impl ToString for DebugLevel {
    fn to_string(&self) -> String {
        match self {
            DebugLevel::None => "-g0",
            DebugLevel::Minimal => "-g1",
            DebugLevel::Full => "-g"
        }.to_owned()
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub enum OptimizeLevel {
    /// Most optimizations are completely disabled.
    #[serde(rename = "none", alias = "O0", alias = "0")]
    #[default]
    None,
    /// Optimize debugging experience.
    #[serde(rename = "debug", alias = "Og", alias = "g")]
    Debug,
    /// Optimize. Optimizing compilation takes somewhat more time, and a lot more memory for a large function.
    #[serde(rename = "level-1", alias = "O1", alias = "1")]
    Optimize,
    /// Optimize even more. GCC performs nearly all supported optimizations that do not involve a space-speed tradeoff.
    #[serde(rename = "level-2", alias = "O2", alias = "2")]
    OptimizeMore,
    /// Optimize yet more.
    #[serde(rename = "level-3", alias = "O3", alias = "3")]
    OptimizeYetMore,
    /// Optimize for size. -Os enables all -O2 optimizations except those that often increase code size.
    #[serde(rename = "size", alias = "Os", alias = "s")]
    OptimizeSize,
    /// Optimize aggressively for size rather than speed.
    #[serde(rename = "size-aggressive", alias = "Oz", alias = "z")]
    OptimizeSizeAggr,
    /// Optimize for speed, disregard strict standards compliance.
    #[serde(rename = "fast", alias = "Ofast")]
    OptimizeFast
}

impl ToString for OptimizeLevel {
    fn to_string(&self) -> String {
        match self {
            OptimizeLevel::None => "-O0",
            OptimizeLevel::Debug => "-Og",
            OptimizeLevel::Optimize => "-O1",
            OptimizeLevel::OptimizeMore => "-O2",
            OptimizeLevel::OptimizeYetMore => "-O3",
            OptimizeLevel::OptimizeSize => "-Os",
            OptimizeLevel::OptimizeSizeAggr => "-Oz",
            OptimizeLevel::OptimizeFast => "-Ofast"
        }.to_owned()
    }
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

impl Profiles {
    /// Get the release profile as compiler arguments.
    pub fn release(&self) -> Vec<String> {
        // TODO: make these statements better...
        let opt_level = self.release.as_ref().map_or(Some(OptimizeLevel::OptimizeYetMore), |p| p.clone().opt_level.map_or(Some(OptimizeLevel::OptimizeYetMore), |f| Some(f))).unwrap();
        let debug = self.release.as_ref().map_or(Some(DebugLevel::None), |p| p.clone().debug.map_or(Some(DebugLevel::None), |f| Some(f))).unwrap();

        vec![opt_level.to_string(), debug.to_string()]
    }

    /// Get the debug profile as compiler arguments.
    pub fn debug(&self) -> Vec<String> {
        // TODO: make these statements better...
        let opt_level = self.release.as_ref().map_or(Some(OptimizeLevel::Debug), |p| p.clone().opt_level.map_or(Some(OptimizeLevel::Debug), |f| Some(f))).unwrap();
        let debug = self.release.as_ref().map_or(Some(DebugLevel::Full), |p| p.clone().debug.map_or(Some(DebugLevel::Full), |f| Some(f))).unwrap();

        vec![opt_level.to_string(), debug.to_string()]
    }
}
