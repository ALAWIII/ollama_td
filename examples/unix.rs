use ollama_td::*;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    let d_location = Path::new(".");

    let o_d_bin = o_d_bin(d_location).await?;
    let o_d_zip = o_d_zip(d_location).await?;

    assert_eq!(o_d_bin.to_str().unwrap(), "./ollama-darwin.tgz");
    assert_eq!(o_d_zip.to_str().unwrap(), "./Ollama-darwin.zip");
    Ok(())
}

// this example attempts to download the ```ollama-darwin``` binary version !!!
async fn o_d_bin(d_location: &Path) -> Result<PathBuf> {
    let u_bin = Platform::Unix(Unix::DarwinBin);

    let o_bin = OllamaDownload::builder()?
        .platform(u_bin)
        .tag_version(TVersion::Latest) //you can always set it to the latest version!!
        .d_location(d_location)
        .build()?;
    download(o_bin).await
}

// this example attempts to download the ```Ollama-darwin.zip``` CLI compressed version !!!
async fn o_d_zip(d_location: &Path) -> Result<PathBuf> {
    let u_zip = Platform::Unix(Unix::DarwinZip);

    let o_zip = OllamaDownload::builder()?
        .platform(u_zip)
        .tag_version(TVersion::Tag("v0.5.7".to_owned())) // you can specify the tag version!!
        .d_location(d_location)
        .build()?;
    download(o_zip).await
}
