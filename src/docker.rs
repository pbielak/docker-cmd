/// Wrapper for Docker client
use shiplift::Docker;

use crate::container_info::ContainerInfo;

pub struct DockerClientWrapper {
    docker: Docker,
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

use shiplift::errors::Error;

impl From<Error> for DockerClientError {
    fn from(e: Error) -> Self {
        match e {
            Error::Hyper(e_msg) => {
                let e_msg: String = e_msg.to_string();
                if e_msg.contains("an error occurred trying to connect: Connection refused") {
                    return DockerClientError::NoDockerDaemonRunning;
                }

                DockerClientError::Other {
                    msg: e_msg.to_string(),
                }
            }
            _ => DockerClientError::Other { msg: e.to_string() },
        }
    }
}

impl DockerClientWrapper {
    pub fn new(docker_address: &str) -> DockerClientWrapper {
        if !docker_address.is_empty() {
            std::env::set_var("DOCKER_HOST", docker_address);
        }

        DockerClientWrapper {
            docker: Docker::new(),
        }
    }

    pub fn get_containers(self) -> Result<Vec<ContainerInfo>, DockerClientError> {
        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let containers = runtime.block_on(self.docker.containers().list(&Default::default()));

        let container_info: Vec<ContainerInfo> = containers?
            .into_iter()
            .map(|c| ContainerInfo::new(&c))
            .collect();

        if container_info.is_empty() {
            return Err(DockerClientError::NoRunningContainers);
        }

        Ok(container_info)
    }
}
