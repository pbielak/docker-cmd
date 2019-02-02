/*
docker-cmd command line tool
*/
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

mod args;
mod container_info;
mod docker;
mod exec;
mod tui;

use docker::DockerClientWrapper;
use exec::exec_cmd;
use tui::TUI;

fn main() {
    let args = args::CliArgs::from_args();

    let client = DockerClientWrapper::new(&args.docker_address);
    let containers = client.get_containers();

    if containers.is_empty() {
        println!("No running containers");
        std::process::exit(0);
    }

    let tui = TUI::new(&containers);
    let container = tui.get_selected_container();

    exec_cmd(&container.id, &args.command);
}
