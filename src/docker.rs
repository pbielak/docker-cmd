/// Wrapper for Docker client
use std::process::Command;

use crate::container_info::ContainerInfo;

pub fn get_containers() -> Result<Vec<ContainerInfo>, DockerClientError> {
    let cmd_out_raw = Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("{{.ID}};{{.Image}};{{.Command}};{{.Status}}")
        .output()?;

    if !cmd_out_raw.status.success() {
        let stderr = String::from_utf8(cmd_out_raw.stderr)?;
        return Err(DockerClientError::from(stderr));
    }

    let stdout_raw = String::from_utf8(cmd_out_raw.stdout)?;
    let stdout = stdout_raw.trim();

    if stdout.is_empty() {
        return Err(DockerClientError::NoRunningContainers);
    }

    let container_info: Vec<ContainerInfo> = stdout
        .split('\n')
        .map(|line| {
            let data: Vec<&str> = line.split(';').collect();
            ContainerInfo::new(
                data[0].to_string(),
                data[1].to_string(),
                data[2].to_string(),
                data[3].to_string(),
            )
        })
        .collect();

    if container_info.is_empty() {
        return Err(DockerClientError::NoRunningContainers);
    }

    Ok(container_info)
}

#[derive(Debug, Display)]
pub enum DockerClientError {
    #[display(fmt = "Docker daemon is not running")]
    NoDockerDaemonRunning,

    #[display(fmt = "No containers are running")]
    NoRunningContainers,

    #[display(fmt = "Unknown error occurred: {}", msg)]
    Other { msg: String },
}

impl From<String> for DockerClientError {
    fn from(s: String) -> Self {
        if s.contains("Cannot connect to the Docker daemon at") {
            return DockerClientError::NoDockerDaemonRunning;
        }

        DockerClientError::Other { msg: s }
    }
}

impl From<std::string::FromUtf8Error> for DockerClientError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        DockerClientError::Other { msg: e.to_string() }
    }
}

impl From<std::io::Error> for DockerClientError {
    fn from(e: std::io::Error) -> Self {
        DockerClientError::Other { msg: e.to_string() }
    }
}
