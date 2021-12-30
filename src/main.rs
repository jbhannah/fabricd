mod config;
mod error;

use crate::config::{read_config, write_lock, Config};
use crate::error::Error;

use async_zip::read::seek::ZipFileReader;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = read_config().await?;
    println!(
        "Looking for Minecraft version {:?}",
        config.minecraft.version
    );

    let mut installer = Command::new("java");

    installer
        .arg("-jar")
        .arg("fabric-installer-0.10.2.jar")
        .arg("server")
        .arg("-downloadMinecraft");
    installer.stdout(Stdio::piped());

    let mut child = installer.spawn().expect("failed to spawn command");
    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");
        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        println!("{}", line);
    }

    let mut lock = Config::new();

    let mut server_jar = File::open("server.jar").await?;
    let mut zip = ZipFileReader::new(&mut server_jar).await?;

    lock.minecraft.version = if let Some((index, _)) = zip.entry("version.json") {
        let mut version_file = zip.entry_reader(index).await?;
        let mut version_str = String::new();

        version_file.read_to_string(&mut version_str).await?;
        let version_data = json::parse(&version_str)?;

        Some(version_data["name"].to_string())
    } else {
        config.minecraft.version
    };

    write_lock(&lock).await?;

    Ok(())
}
