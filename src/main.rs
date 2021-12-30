mod config;
mod error;
mod server;

use crate::config::{read_config, read_lock, write_lock};
use crate::error::Error;
use crate::server::version::read_version;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use std::path::Path;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = read_config().await?;
    let mut lock = read_lock().await?;

    let download_launcher =
        !(Path::new("server.jar").exists() && Path::new("fabric-server-launch.jar").exists());

    println!(
        "Looking for Minecraft version {:?}",
        config.minecraft.version
    );

    if config.minecraft.version != lock.minecraft.version || download_launcher {
        let mut installer = Command::new("java");

        installer
            .arg("-jar")
            .arg("fabric-installer-0.10.2.jar")
            .arg("server")
            .arg("-downloadMinecraft");

        if let Some(ref version) = config.minecraft.version {
            installer.arg("-mcversion").arg(version);
        }

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

        lock.minecraft.version = read_version().await?.or(config.minecraft.version);
    }

    write_lock(&lock).await?;

    Ok(())
}
