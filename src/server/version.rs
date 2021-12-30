use crate::server::error::VersionError;

use async_zip::read::seek::ZipFileReader;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn read_version() -> Result<Option<String>, VersionError> {
    let mut file = File::open("server.jar").await?;
    let mut zip = ZipFileReader::new(&mut file).await?;

    if let Some((index, _)) = zip.entry("version.json") {
        let mut version_file = zip.entry_reader(index).await?;
        let mut version_string = String::new();

        version_file.read_to_string(&mut version_string).await?;
        let version_data = json::parse(&version_string)?;

        if let Some(version) = version_data["name"].as_str() {
            println!("Read version {} from server.jar", version);
            Ok(Some(version.to_string()))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
