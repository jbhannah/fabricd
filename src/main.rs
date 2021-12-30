mod config;
mod error;

use crate::config::{read_config, write_lock, Config};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    lock.minecraft.version = Some("1.18.2".to_string());
    write_lock(&lock).await?;

    Ok(())
}
