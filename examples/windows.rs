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

    download(o_download, None).await
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
