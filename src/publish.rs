use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Publish {
    /// Define the group you want the project to be published in. By default,
    /// the project will be published in your user group.
    #[structopt(long = "group")]
    group: Option<String>,

    /// Define the message for the initial commit. A generic message will be used by default.
    #[structopt(long = "message")]
    message: Option<String>,
}
impl Publish {
    pub fn publish(&self) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
}
