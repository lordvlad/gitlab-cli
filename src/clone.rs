use std::error::Error;
use std::ffi::OsString;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::TrailingVarArg"))]
pub struct GitClone {
    /// The project path, i.e. namespace and project name, of the project you want to clone.
    project: String,

    /// Arguments to git command
    #[structopt(parse(from_os_str))]
    args: Vec<OsString>,
}
impl GitClone {
    pub fn git_clone(&self) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
}
