// Command line args

#[derive(StructOpt, Debug)]
pub struct CliArgs {
    #[structopt(default_value = "/bin/bash")]
    pub command: String,
}
