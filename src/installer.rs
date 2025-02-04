use tokio::fs;
use std::path::Path;
use std::process::Command;

pub struct DriverInstaller;

impl DriverInstaller {
    pub async fn download_driver(url: &str, path: &Path) -> Result<(), InstallError> {
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        fs::write(path, content).await?;
        Ok(())
    }

    pub async fn install_driver(path: &Path) -> Result<(), InstallError> {
        let status = Command::new("powershell")
            .args(&[
                "-Command",
                &format!("Start-Process -Wait -FilePath '{}' -ArgumentList '/S /quiet'", 
                    path.display())
            ])
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err(InstallError::InstallFailed)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InstallError {
    #[error("Download failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Installation failed")]
    InstallFailed,
}