use crate::*;
use octocrab::{models::repos::Asset, Octocrab};
use std::io::copy;
use std::path::PathBuf;
use std::{fs::File, path::Path};

/// Accepts OllamaDownload object and after successfully downloading your tool it will return the path to where your tool is stored!!!
///
/// As an example returns: ```./ollama-linux-amd64-rocm.tgz``` , with the full name of the tool at the end !!!
pub async fn download(mut o_download: OllamaDownload) -> OResult<PathBuf> {
    let platform = o_download.get_platform();
    let tag_version = o_download.get_tag_version();
    let o_asset = fetch_ollama_asset(tag_version, platform).await?;
    let d_location = o_download.get_d_location();
    let downloaded = download_ollama_tool(o_asset, d_location).await?;
    Ok(downloaded)
}

/// accepts the tag_version and the platform and returns the associated Asset that describes the copy of the tool you want !!
///
/// # FAILS
/// when you specify non-existed Tag !
async fn fetch_ollama_asset(tag_version: &TVersion, platform: &Platform) -> OResult<Asset> {
    let o = Octocrab::builder().build()?;
    let owner = "ollama";
    let repo = "ollama";
    let info = o.repos(owner, repo);
    let release = info.releases();
    let tag = match tag_version {
        TVersion::Latest => release.get_latest().await,
        TVersion::Tag(t) => release.get_by_tag(t).await,
    }?
    .assets
    .into_iter()
    .find(|a| a.name.ends_with(&platform.to_string()))
    .unwrap();
    Ok(tag)
}

/// downloads the specified version of the ollama tool.
///
/// # FAILS
/// when either connection problems , file path not correct OR failing when writing and coping the bytes of the downloaded file into the local non-voltile buffer.
async fn download_ollama_tool(asset: Asset, d_path: &mut Path) -> OResult<PathBuf> {
    let full_path = d_path.join(asset.name);
    let tool = reqwest::get(asset.browser_download_url).await?;
    let mut file = File::create(&full_path)?;
    copy(&mut tool.bytes().await?.as_ref(), &mut file)?;
    Ok(full_path)
}

//------------------------------------------- tests ------------
#[cfg(test)]
mod d_tests {
    use super::{fetch_ollama_asset, Linux, OResult, Platform, TVersion};

    #[tokio::test]
    async fn fetch_o_asset() -> OResult<()> {
        let tag = TVersion::Tag("v0.5.7".to_owned());
        let plat = Platform::Linux(Linux::X86 { rocm: true });
        let downloaded_tool = fetch_ollama_asset(&tag, &plat).await?;
        assert_eq!(downloaded_tool.name, "ollama-linux-amd64-rocm.tgz");
        Ok(())
    }
    #[tokio::test]
    async fn fetch_o_asset_no_tag() {
        let tag = TVersion::Tag("v0.555".to_owned());
        let plat = Platform::Linux(Linux::X86 { rocm: true });
        let downloaded_tool = fetch_ollama_asset(&tag, &plat).await;
        assert!(downloaded_tool.is_err());
    }
}
