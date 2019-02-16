/// Command line args

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct CliArgs {
    #[structopt(default_value = "/bin/bash")]
    pub command: String,
}
