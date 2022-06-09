mod config;
mod error;
mod fabric;
mod server;
mod util;

use crate::config::{read_config, read_lock, write_lock};
use crate::error::Error;
use crate::fabric::installer::download_installer;
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

    let mut run_installer = false;

    let get_installer = tokio::spawn(async move {
        installer::latest_version()
            .await
            .expect("could not get installer version")
    });

    let get_loader = tokio::spawn(async move {
        loader::latest_version()
            .await
            .expect("could not get loader version")
    });

    let (latest_installer_version, latest_loader_version) = try_join!(get_installer, get_loader)?;
    println!("Latest Fabric installer: {:?}", latest_installer_version);
    println!("Latest Fabric loader: {:?}", latest_loader_version);

    let installer_version = match config.fabric.installer.version.as_deref() {
        Some("latest") => latest_installer_version,
        Some(version) => version.to_string(),
        None => latest_installer_version,
    };

    let installer_filename = format!("fabric-installer-{}.jar", installer_version);

    if (config.fabric.installer.version.as_deref() != Some("latest")
        && config.fabric.installer.version != lock.fabric.installer.version)
        || !Path::new(&installer_filename).exists()
    {
        download_installer(&installer_version).await?;
        lock.fabric.installer.version = Some(installer_version);
        run_installer = true;
    }

    run_installer = run_installer
        || !(Path::new("server.jar").exists() && Path::new("fabric-server-launch.jar").exists());

    println!(
        "Looking for Minecraft version {:?}",
        config.minecraft.version
    );

    if config.minecraft.version != lock.minecraft.version || run_installer {
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
