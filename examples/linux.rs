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
