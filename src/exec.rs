// Process transformation to actual docker call
use std::ffi::CString;

use nix::unistd::*;

pub fn exec_cmd(container_id: &str, command: &str) {
    let docker_binary: &str = "/usr/local/bin/docker";

    let raw_args: Vec<&str> = vec!["docker", "exec", "-it", container_id, command];

    let cmd = CString::new(docker_binary).unwrap();
    let args: Vec<CString> = raw_args
        .into_iter()
        .map(|arg| CString::new(arg).unwrap())
        .collect();

    match execv(&cmd, &args[..]) {
        Ok(_) => unreachable!(),
        Err(e) => println!("Error on execv: {:?}", e),
    }
}
