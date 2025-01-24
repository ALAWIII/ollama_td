# ollama_td ![License: MIT](https://img.shields.io/badge/license-MIT-blue) [![ollama_td on crates.io](https://img.shields.io/crates/v/ollama_td)](https://crates.io/crates/ollama_td) [![ollama_td on docs.rs](https://docs.rs/ollama_td/badge.svg)](https://docs.rs/ollama_td)

ollama_td : `ollama tool download` crate , which is a crate used exclusively to download the ollama command line tool or the binary itself ,

this is Not a crate to download the models, but to download the tool that is used to manage and download the models !!!

## Examples

Different platforms have different several options available .

### Windows

```rust
use ollama_td::*;
use std::path::Path;

#[tokio::main]
async fn main() -> OResult<()> {
    let w_x86 = Platform::Windows(Windows::X86);
    let w_arm = Platform::Windows(Windows::Arm);
    let w_exe = Platform::Windows(Windows::BinExe);

    let d_location = Path::new(".");

    let o_d_x86 = OllamaDownload::builder()?
        .platform(w_x86)
        .d_location(d_location)
        .tag_version(TVersion::Latest)
        .build()?;

    let o_d_arm = OllamaDownload::builder()?
        .platform(w_arm)
        .d_location(d_location)
        .tag_version(TVersion::Tag("v0.5.7".to_string()))
        .build()?;

    let o_d_exe = OllamaDownload::builder()?
        .platform(w_exe)
        .d_location(d_location)
        .tag_version(TVersion::Tag("v0.5.7".to_string()))
        .build()?;

    let down_x86 = download(o_d_x86).await?; // returns the path when successfully downloading the tool!
    let down_arm = download(o_d_arm).await?;
    let down_exe = download(o_d_exe).await?;

    assert_eq!(down_x86.to_str().unwrap(), "./ollama-windows-amd64.zip ");
    assert_eq!(down_arm.to_str().unwrap(), "./ollama-windows-arm64.zip");
    assert_eq!(down_exe.to_str().unwrap(), "./OllamaSetup.exe");
    Ok(())
}

```

### Unix

```rust
use ollama_td::*;
use std::path::Path;

#[tokio::main]
async fn main() -> OResult<()> {
   let u_bin = Platform::Unix(Unix::DarwinBin);
   let u_zip = Platform::Unix(Unix::DarwinZip);
   let d_location = Path::new(".");

   let o_bin = OllamaDownload::builder()?
       .platform(u_bin)
       .tag_version(TVersion::Latest)
       .d_location(d_location)
       .build()?;

   let o_zip = OllamaDownload::builder()?
       .platform(u_zip)
       .tag_version(TVersion::Tag("v0.5.7".to_owned()))
       .d_location(d_location)
       .build()?;

   let o_d_bin = download(o_bin).await?;
   let o_d_zip = download(o_zip).await?;

   assert_eq!(o_d_bin.to_str().unwrap(), "./ollama-darwin");
   assert_eq!(o_d_zip.to_str().unwrap(), "./Ollama-darwin.zip");
   Ok(())
}

```

### Linux

```rust
use ollama_td::*;
use std::path::Path;

#[tokio::main]
async fn main() -> OResult<()> {
   let l_x86 = Platform::Linux(Linux::X86 { rocm: false });
   let l_x86_rocm = Platform::Linux(Linux::X86 { rocm: true });
   let l_arm = Platform::Linux(Linux::Arm(LinuxArm::Arm));
   let l_arm_jet5 = Platform::Linux(Linux::Arm(LinuxArm::Jetpack5));
   let l_arm_jet6 = Platform::Linux(Linux::Arm(LinuxArm::Jetpack6));

   let d_location = Path::new(".");

   let o_x86 = OllamaDownload::builder()?
       .platform(l_x86)
       .tag_version(TVersion::Latest)
       .d_location(d_location)
       .build()?;
   let o_x86_rocm = OllamaDownload::builder()?
       .platform(l_x86_rocm)
       .tag_version(TVersion::Latest)
       .d_location(d_location)
       .build()?;
   let o_arm = OllamaDownload::builder()?
       .platform(l_arm)
       .tag_version(TVersion::Latest)
       .d_location(d_location)
       .build()?;
   let o_arm_jet5 = OllamaDownload::builder()?
       .platform(l_arm_jet5)
       .tag_version(TVersion::Latest)
       .d_location(d_location)
       .build()?;
   let o_arm_jet6 = OllamaDownload::builder()?
       .platform(l_arm_jet6)
       .tag_version(TVersion::Latest)
       .d_location(d_location)
       .build()?;

   let o_d_x86 = download(o_x86).await?;
   let o_d_x86_rocm = download(o_x86_rocm).await?;
   let o_d_arm = download(o_arm).await?;
   let o_d_arm_jet5 = download(o_arm_jet5).await?;
   let o_d_arm_jet6 = download(o_arm_jet6).await?;
   //--------------------testing
   assert_eq!(o_d_x86.to_str().unwrap(), "./ollama-linux-amd64.tgz");
   assert_eq!(
       o_d_x86_rocm.to_str().unwrap(),
       "./ollama-linux-amd64-rocm.tgz"
   );
   //---------------arm versions--------------
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

```
