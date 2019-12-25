// Process transformation to actual docker call
use std::ffi::CString;

use nix::unistd::execv;
use which::which;

pub fn exec_cmd(container_id: &str, command: &str) {
    let docker_binary: String = find_binary("docker");

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

fn find_binary(binary: &str) -> String {
    match which(binary) {
        Ok(bin) => bin.into_os_string().into_string().unwrap(),
        Err(e) => panic!("\"{:?}\" binary not found: {:?}", binary, e),
    }
}
