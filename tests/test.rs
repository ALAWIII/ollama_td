use ollama_td::*;
use reqwest::Response;
use std::fs::File;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};
//------------------
#[test]
fn linux() {
    let l_x86 = Platform::Linux(Linux::X86 { rocm: false });
    let l_x86_rocm = Platform::Linux(Linux::X86 { rocm: true });
    let l_arm = Platform::Linux(Linux::Arm(LinuxArm::Arm));
    let l_arm_jet5 = Platform::Linux(Linux::Arm(LinuxArm::Jetpack5));
    let l_arm_jet6 = Platform::Linux(Linux::Arm(LinuxArm::Jetpack6));

    assert_eq!(l_x86.to_string(), "amd64.tgz".to_string());
    assert_eq!(l_x86_rocm.to_string(), "amd64-rocm.tgz".to_string());
    assert_eq!(l_arm.to_string(), "arm64.tgz"); //jetpack5.tgz
    assert_eq!(l_arm_jet5.to_string(), "jetpack5.tgz");
    assert_eq!(l_arm_jet6.to_string(), "jetpack6.tgz");
}
//------------------
#[test]
fn windows() {
    let w_x86 = Platform::Windows(Windows::X86);
    let w_arm = Platform::Windows(Windows::Arm);
    let w_exe = Platform::Windows(Windows::BinExe);

    assert_eq!(w_x86.to_string(), "amd64.zip".to_string());
    assert_eq!(w_arm.to_string(), "arm64.zip".to_string());
    assert_eq!(w_exe.to_string(), "exe".to_string());
}

//-------------------
#[test]
fn unix() {
    let u_bin = Platform::Unix(Unix::DarwinBin);
    let u_zip = Platform::Unix(Unix::DarwinZip);
    assert_eq!(u_bin.to_string(), "darwin.tgz".to_owned());
    assert_eq!(u_zip.to_string(), "darwin.zip".to_owned());
}
//-----------------------------------
#[test]
fn ollama_d_default() {
    let o = OllamaDownload::default();
    assert_eq!(o.get_platform().to_string(), "amd64.tgz");
    assert_eq!(o.get_tag_version().to_string(), "Latest");
}
//-------------------------------
#[test]
fn ollama_d_builder() {
    let o = OllamaDownload::builder()
        .unwrap()
        .d_location(Path::new("."))
        .platform(Platform::Linux(Linux::X86 { rocm: false }))
        .tag_version(TVersion::Tag("v0.5.7".to_owned()))
        .build()
        .unwrap();
    assert_eq!(o.to_string(), "amd64.tgz v0.5.7 .");
}
//----------------------------------------------------------

#[tokio::test]
//#[ignore = "It successfully passed but consumes time on every excution of all tests!!"]
async fn download_o_tool() -> OResult<()> {
    let o_download = OllamaDownload::builder()
        .unwrap()
        .tag_version(TVersion::Latest)
        .platform(Platform::Unix(Unix::DarwinZip))
        .d_location(Path::new("."))
        .build()
        .unwrap();
    let downloaded = download(o_download).await;
    assert_eq!(downloaded?.to_str().unwrap(), "./Ollama-darwin.zip");
    Ok(())
}
//----------------------------------

async fn download_custom_helper(mut res: Response, full_path: &mut Path) -> OResult<PathBuf> {
    let content_length = res.content_length().unwrap_or(1) as f64;
    let mut file = File::create(&full_path)?;
    let mut recived = 0.0;
    while let Some(c) = res.chunk().await? {
        recived += c.len() as f64;
        print!("\r{:.2}", (recived / content_length) * 100.0);
        file.write_all(&c)?;
        stdout().flush()?;
    }
    file.flush()?;
    Ok(full_path.to_path_buf())
}

#[tokio::test]
async fn test_download_custom() -> OResult<()> {
    let o_download = OllamaDownload::builder()
        .unwrap()
        .tag_version(TVersion::Latest)
        .platform(Platform::Unix(Unix::DarwinZip))
        .d_location(Path::new("."))
        .build()
        .unwrap();
    let downloaded = download_customize(o_download, download_custom_helper).await;
    assert_eq!(downloaded?.to_str().unwrap(), "./Ollama-darwin.zip");
    Ok(())
}
