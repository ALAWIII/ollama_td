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
