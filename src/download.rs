use crate::*;
use octocrab::{Octocrab, models::repos::Asset};
use reqwest::Response;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Accepts OllamaDownload object and after successfully downloading your tool it will return the path to where your tool is stored!!!
///
/// By default ,when you specify f_stream to None , it will use default quick method to accomplish the download process.
///
/// You can specify ```f_stream``` function that will accept Response as an argument ,then you can specify how you would like to handle the download process ,and if you want to stream the progress of the download file.
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

/// used to give you the freedom to customize the download process , for example if you want to plug a GUI progress bar that views the download progress.
pub async fn download_customize(
    mut o_download: OllamaDownload,
    f_stream: impl AsyncFnOnce(Response, &mut Path) -> OResult<PathBuf>,
) -> OResult<PathBuf> {
    let platform = o_download.get_platform();
    let tag_version = o_download.get_tag_version();
    let o_asset = fetch_ollama_asset(tag_version, platform).await?;
    let tool = reqwest::get(o_asset.browser_download_url)
        .await?
        .error_for_status()?;
    let mut full_path = o_download.get_d_location().join(o_asset.name);

    f_stream(tool, &mut full_path).await
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
/// by default ,when you specify f_stream to None , it will use default quick method to accomplish the download process.
///
/// you can specify f_stream function that will accept Response as an argument ,then you can specify how you would like to handle the download process ,and if you want to stream the progress of the download file.
/// # FAILS
/// when either connection problems , file path not correct OR failing when writing and coping the bytes of the downloaded file into the local non-voltile buffer.
async fn download_ollama_tool(asset: Asset, d_path: &mut Path) -> OResult<PathBuf> {
    let full_path = d_path.join(asset.name);
    let tool = reqwest::get(asset.browser_download_url)
        .await?
        .error_for_status()?;

    write_to_disk(tool, full_path).await
}

async fn write_to_disk(mut tool: Response, full_path: PathBuf) -> OResult<PathBuf> {
    let mut file = File::create(&full_path).await?;

    while let Some(chunk) = tool.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    Ok(full_path)
}

//------------------------------------------- tests ------------
#[cfg(test)]
mod d_tests {
    use super::{Linux, OResult, Platform, TVersion, fetch_ollama_asset};

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
