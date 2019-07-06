use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Status {
    /// Define the project you want to see the CI status of. By default,
    /// gitlab can figure out the project if you are in a project directory.
    #[structopt(long = "project")]
    project: Option<String>,
}

impl Status {
    pub fn status(&self) -> Result<(), Box<Error>> {
        unimplemented!();
    }
}
