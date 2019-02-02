/*
Wrapper for Docker client
*/
use shiplift::Docker;

use crate::container_info::ContainerInfo;

pub struct DockerClientWrapper {
    docker: Docker,
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

    pub fn get_containers(self) -> Vec<ContainerInfo> {
        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let res = runtime.block_on(self.docker.containers().list(&Default::default()));

        match res {
            Ok(containers) => containers
                .into_iter()
                .map(|c| ContainerInfo::new(&c))
                .collect(),
            Err(err) => {
                println!("Error occurred when getting container list: {}", err);
                std::process::exit(1)
            }
        }
    }
}
