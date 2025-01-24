use crate::Platform;
use crate::*;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use sys_info::os_type;

pub type OResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
/// provide the tag number or choose to set it to Latest which will choose that latest up-to-data available Asset!!!
pub enum TVersion {
    Tag(String),
    Latest,
}
impl TVersion {
    pub fn get_tag(&self) -> String {
        match self {
            Self::Latest => "Latest",
            Self::Tag(t) => t,
        }
        .to_string()
    }
}
impl Display for TVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_tag())
    }
}
/// Accepts the platform (Operating system) and the download location.
///
/// by default the download location will be in the same location where your final binary will be excuted.
///
/// by default the platform will be your OS (Windows,Linux,Unix) and X86 architecture.
///
/// by default the tag version will be set to Latest.
pub struct OllamaDownload {
    platform: Platform,
    d_location: PathBuf,
    tag_version: TVersion,
}
impl OllamaDownload {
    pub fn builder() -> OResult<OllamaDownloadBuilder> {
        OllamaDownloadBuilder::new()
    }
}
impl Default for OllamaDownload {
    fn default() -> Self {
        OllamaDownloadBuilder::new().unwrap().build().unwrap()
    }
}
impl OllamaDownload {
    pub fn get_platform(&self) -> &Platform {
        &self.platform
    }
    pub fn get_tag_version(&self) -> &TVersion {
        &self.tag_version
    }
    pub fn get_d_location(&mut self) -> &mut PathBuf {
        &mut self.d_location
    }
}
//------------------
impl Display for OllamaDownload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.platform)?;
        write!(f, "{} ", self.tag_version)?;
        write!(f, "{}", self.d_location.to_str().unwrap())
    }
}
//----------------------------------------------------------------------------
#[derive(Debug)]
/// used to build the OllamaDownload struct with nice minimal functionality to set the fields!!!
pub struct OllamaDownloadBuilder {
    platform: Platform,
    d_location: PathBuf,
    tag_version: TVersion,
}

impl OllamaDownloadBuilder {
    pub fn new() -> OResult<Self> {
        let current_folder = env::current_exe()?;
        Ok(Self {
            platform: get_os_platform()?,
            d_location: current_folder.parent().unwrap().to_path_buf(),
            tag_version: TVersion::Latest,
        })
    }
    /// to set the download location
    pub fn d_location(mut self, path: &Path) -> Self {
        self.d_location = path.to_path_buf();
        self
    }
    /// to set the platform (e.g Platform::Linux(Linux::X86 { rocm: false }))
    pub fn platform(mut self, platform: Platform) -> Self {
        self.platform = platform;
        self
    }
    /// critical to set the tag verion , it defaults to latest , or specify the version
    ///
    /// #Example
    ///  TVersion::Tag("v0.5.7".to_string())
    pub fn tag_version(mut self, tversion: TVersion) -> Self {
        self.tag_version = tversion;
        self
    }
    pub fn build(self) -> OResult<OllamaDownload> {
        let odownload = OllamaDownload {
            platform: self.platform,
            d_location: self.d_location,
            tag_version: self.tag_version,
        };
        Ok(odownload)
    }
}
//-------------------
fn get_os_platform() -> OResult<Platform> {
    let os_info = os_type()?;
    let platform = match os_info.as_str() {
        "Linux" => Platform::Linux(Linux::X86 { rocm: false }),
        "Darwin" => Platform::Unix(Unix::DarwinZip),
        _ => Platform::Windows(Windows::X86),
    };
    Ok(platform)
}
