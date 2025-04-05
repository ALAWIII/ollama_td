use ollama_td::*;
use reqwest::Response;
use std::fs::File;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
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
) -> Result<PathBuf> {
    let o_download = OllamaDownload::builder()?
        .platform(platform)
        .d_location(d_location)
        .tag_version(tag_version)
        .build()?;

    download_customize(o_download, download_custom_helper).await
}

// downloads [ollama-windows-amd64.zip]
async fn o_d_x86(d_location: &Path) -> Result<PathBuf> {
    let platform = Platform::Windows(Windows::X86);
    download_ollama(d_location, platform, TVersion::Latest).await
}
// downloads [ollama-windows-arm64.zip]
async fn o_d_arm(d_location: &Path) -> Result<PathBuf> {
    let platform = Platform::Windows(Windows::Arm);
    download_ollama(d_location, platform, TVersion::Tag("v0.5.7".to_string())).await
}

// downloads [OllamaSetup.exe]
async fn o_d_exe(d_location: &Path) -> Result<PathBuf> {
    let platform = Platform::Windows(Windows::BinExe);
    download_ollama(d_location, platform, TVersion::Tag("v0.5.7".to_string())).await
}

// is used with download_custom function , here we stream to the disk storage and to the stdout!!
async fn download_custom_helper(mut res: Response, full_path: &mut Path) -> Result<PathBuf> {
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
