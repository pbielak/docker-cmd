/// docker-cmd command line tool
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

mod args;
mod container_info;
mod docker;
mod exec;
mod tui;

fn main() {
    let args = args::CliArgs::from_args();

    let client = docker::DockerClientWrapper::new(&args.docker_address);

    match client.get_containers() {
        Err(err) => println!("{}", err),
        Ok(containers) => {
            let container = tui::get_selected_container(&containers);
            exec::exec_cmd(&container.id, &args.command);
        }
    }
}
