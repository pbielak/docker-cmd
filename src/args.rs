/// Command line args

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct CliArgs {
    #[structopt(long = "docker-address", default_value = "")]
    pub docker_address: String,

    #[structopt(short, long, default_value = "/bin/bash")]
    pub command: String,
}
