# ollama_td ![License: MIT](https://img.shields.io/badge/license-MIT-blue) [![ollama_td on crates.io](https://img.shields.io/crates/v/ollama_td)](https://crates.io/crates/ollama_td) [![ollama_td on docs.rs](https://docs.rs/ollama_td/badge.svg)](https://docs.rs/ollama_td)

ollama_td : `ollama tool download` crate , which is a crate used exclusively to download the ollama command line tool or the binary itself ,

this is Not a crate to download the models, but to download the tool that is used to manage and download the models !!!

you can automate the download of the compressed (e.g. zip or tgz) CLI tool and

automatically unpack and place it where ever you want .

## Examples

Different platforms have different several options available .

### Windows

you have three options :

* ollama-windows-amd64.zip
* ollama-windows-arm64.zip
* OllamaSetup.exe

```rust
use ollama_td::*;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> OResult<()> {
    let d_location = Path::new(".");

    let down_x86 = o_d_x86(d_location).await?;
    let down_arm = o_d_arm(d_location).await?;
    let down_exe = o_d_exe(d_location).await?;

    assert_eq!(down_x86.to_str().unwrap(), "./ollama-windows-amd64.zip");
    assert_eq!(down_arm.to_str().unwrap(), "./ollama-windows-arm64.zip");
    assert_eq!(down_exe.to_str().unwrap(), "./OllamaSetup.exe");

    Ok(())
}

// general function function to be used among defferent other function examples !!
async fn download_ollama(
    d_location: &Path,
    platform: Platform,
    tag_version: TVersion,
) -> OResult<PathBuf> {
    let o_download = OllamaDownload::builder()?
        .platform(platform)
        .d_location(d_location)
        .tag_version(tag_version)
        .build()?;

    download(o_download).await
}

// downloads [ollama-windows-amd64.zip]
async fn o_d_x86(d_location: &Path) -> OResult<PathBuf> {
    let platform = Platform::Windows(Windows::X86);
    download_ollama(d_location, platform, TVersion::Latest).await
}
// downloads [ollama-windows-arm64.zip]
async fn o_d_arm(d_location: &Path) -> OResult<PathBuf> {
    let platform = Platform::Windows(Windows::Arm);
    download_ollama(d_location, platform, TVersion::Tag("v0.5.7".to_string())).await
}

// downloads [OllamaSetup.exe]
async fn o_d_exe(d_location: &Path) -> OResult<PathBuf> {
    let platform = Platform::Windows(Windows::BinExe);
    download_ollama(d_location, platform, TVersion::Tag("v0.5.7".to_string())).await
}

```

### Unix

you have two options :

* ollama-darwin
* Ollama-darwin.zip

```rust
use ollama_td::*;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> OResult<()> {
   let d_location = Path::new(".");

   let o_d_bin = o_d_bin(d_location).await?;
   let o_d_zip = o_d_zip(d_location).await?;

   assert_eq!(o_d_bin.to_str().unwrap(), "./ollama-darwin");
   assert_eq!(o_d_zip.to_str().unwrap(), "./Ollama-darwin.zip");
   Ok(())
}

// this example attempts to download the ```ollama-darwin``` binary version !!!
async fn o_d_bin(d_location: &Path) -> OResult<PathBuf> {
   let u_bin = Platform::Unix(Unix::DarwinBin);

   let o_bin = OllamaDownload::builder()?
       .platform(u_bin)
       .tag_version(TVersion::Latest) //you can always set it to the latest version!!
       .d_location(d_location)
       .build()?;
   download(o_bin).await
}

// this example attempts to download the ```Ollama-darwin.zip``` CLI compressed version !!!
async fn o_d_zip(d_location: &Path) -> OResult<PathBuf> {
   let u_zip = Platform::Unix(Unix::DarwinZip);

   let o_zip = OllamaDownload::builder()?
       .platform(u_zip)
       .tag_version(TVersion::Tag("v0.5.7".to_owned())) // you can specify the tag version!!
       .d_location(d_location)
       .build()?;
   download(o_zip).await
}

```

### Linux

you have five options :

* ollama-linux-amd64.tgz
* ollama-linux-amd64-rocm.tgz
* ollama-linux-arm64.tgz
* ollama-linux-arm64-jetpack5.tgz
* ollama-linux-arm64-jetpack6.tgz

```rust
use ollama_td::*;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> OResult<()> {
   let d_location = Path::new(".");

   let o_d_x86 = o_d_x86(d_location).await?;
   let o_d_x86_rocm = o_d_x86_rocm(d_location).await?;
   let o_d_arm = o_d_arm(d_location).await?;
   let o_d_arm_jet5 = o_d_arm_jet5(d_location).await?;
   let o_d_arm_jet6 = o_d_arm_jet6(d_location).await?;

   assert_eq!(o_d_x86.to_str().unwrap(), "./ollama-linux-amd64.tgz");

   assert_eq!(
       o_d_x86_rocm.to_str().unwrap(),
       "./ollama-linux-amd64-rocm.tgz"
   );

   assert_eq!(o_d_arm.to_str().unwrap(), "./ollama-linux-arm64.tgz");

   assert_eq!(
       o_d_arm_jet5.to_str().unwrap(),
       "./ollama-linux-arm64-jetpack5.tgz"
   );

   assert_eq!(
       o_d_arm_jet6.to_str().unwrap(),
       "./ollama-linux-arm64-jetpack6.tgz"
   );

   Ok(())
}

// general function to be reused among other functions!!
async fn download_ollama(d_location: &Path, platform: Platform) -> OResult<PathBuf> {
   let o_download = OllamaDownload::builder()?
       .platform(platform)
       .tag_version(TVersion::Latest)
       .d_location(d_location)
       .build()?;

   download(o_download).await
}

// downloads [ollama-linux-amd64.tgz]
async fn o_d_x86(d_location: &Path) -> OResult<PathBuf> {
   let platform = Platform::Linux(Linux::X86 { rocm: false });
   download_ollama(d_location, platform).await
}

// downloads [ollama-linux-amd64-rocm.tgz]
async fn o_d_x86_rocm(d_location: &Path) -> OResult<PathBuf> {
   let platform = Platform::Linux(Linux::X86 { rocm: true });
   download_ollama(d_location, platform).await
}

// downloads [ollama-linux-arm64.tgz]
async fn o_d_arm(d_location: &Path) -> OResult<PathBuf> {
   let platform = Platform::Linux(Linux::Arm(LinuxArm::Arm));
   download_ollama(d_location, platform).await
}
// downloads [ollama-linux-arm64-jetpack5.tgz]
async fn o_d_arm_jet5(d_location: &Path) -> OResult<PathBuf> {
   let platform = Platform::Linux(Linux::Arm(LinuxArm::Jetpack5));
   download_ollama(d_location, platform).await
}
// downloads [ollama-linux-arm64-jetpack6.tgz]
async fn o_d_arm_jet6(d_location: &Path) -> OResult<PathBuf> {
   let platform = Platform::Linux(Linux::Arm(LinuxArm::Jetpack6));
   download_ollama(d_location, platform).await
}

```
