use ollama_td::*;
use std::path::Path;

#[tokio::main]
async fn main() -> OResult<()> {
    let u_bin = Platform::Unix(Unix::DarwinBin);
    let u_zip = Platform::Unix(Unix::DarwinZip);
    let d_location = Path::new(".");

    let o_bin = OllamaDownload::builder()?
        .platform(u_bin)
        .tag_version(TVersion::Latest)
        .d_location(d_location)
        .build()?;

    let o_zip = OllamaDownload::builder()?
        .platform(u_zip)
        .tag_version(TVersion::Tag("v0.5.7".to_owned()))
        .d_location(d_location)
        .build()?;

    let o_d_bin = download(o_bin).await?;
    let o_d_zip = download(o_zip).await?;

    assert_eq!(o_d_bin.to_str().unwrap(), "./ollama-darwin");
    assert_eq!(o_d_zip.to_str().unwrap(), "./Ollama-darwin.zip");
    Ok(())
}
