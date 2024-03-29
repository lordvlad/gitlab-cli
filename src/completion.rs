use std::error::Error;
use std::io::stdout;
use structopt::StructOpt;

use super::GitlabCli;

#[derive(Debug, StructOpt)]
pub struct Completion {
    #[structopt(default_value = "bash")]
    pub shell: String,
}
impl Completion {
    pub fn completion(&self) -> Result<(), Box<dyn Error>> {
        GitlabCli::clap().gen_completions_to("gitlab", self.shell.parse().unwrap(), &mut stdout());
        Ok(())
    }
}
