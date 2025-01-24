use std::path::Path;

use ollama_td::*;

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
    assert_eq!(u_bin.to_string(), "darwin".to_owned());
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
#[ignore = "It successfully passed but consumes time on every excution of all tests!!"]
async fn donwload_o_tool() {
    let o_download = OllamaDownload::builder()
        .unwrap()
        .tag_version(TVersion::Latest)
        .platform(Platform::Unix(Unix::DarwinBin))
        .d_location(Path::new("."))
        .build()
        .unwrap();
    let downloaded = download(o_download).await.unwrap();
    assert_eq!(downloaded.to_str().unwrap(), "./ollama-darwin");
}
