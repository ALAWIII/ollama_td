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
