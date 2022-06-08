mod config;
mod error;
mod fabric;
mod server;

use crate::config::{read_config, read_lock, write_lock};
use crate::error::Error;
use crate::fabric::{installer, loader};
use crate::server::version::read_version;

use futures::try_join;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use std::path::Path;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = read_config().await?;
    let mut lock = read_lock().await?;

    let get_installer = tokio::spawn(async move {
        let installer_version = installer::latest_version()
            .await
            .expect("could not get installer version");
        println!("Latest Fabric installer: {:?}", installer_version);
    });

    let get_loader = tokio::spawn(async move {
        let loader_version = loader::latest_version()
            .await
            .expect("could not get loader version");
        println!("Latest Fabric loader: {:?}", loader_version);
    });

    try_join!(get_installer, get_loader)?;

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
