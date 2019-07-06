use std::error::Error;
use std::process::{exit, Command};
use std::result::Result::{Err, Ok};

use clap::arg_enum;
use structopt::StructOpt;

use super::gitlab_config;

#[derive(Debug, StructOpt)]
pub struct Configure {
    /// The name of the configuration property you want displayed or changed. If you
    /// pass 'reset' as the key, all configuration options will be reset to defaults.
    /// If you pass 'list' as the key, all configuration options will be printed.
    #[structopt(raw(
        possible_values = "&ConfigureKeys::variants()",
        case_insensitive = "true"
    ))]
    key: ConfigureKeys,
    /// If given, changes the configuration property to the new value. If omitted,
    /// displays the current value.
    value: Option<String>,
}

impl Configure {
    pub fn configure(&self) -> Result<(), Box<Error>> {
        match &self.value {
            Some(val) => Configure::set_value(&self.key, &val),
            None => Configure::print_value(&self.key),
        }
    }

    /// Sets the configuration property identified by 'key' to 'value'.
    fn set_value(key: &ConfigureKeys, value: &str) -> Result<(), Box<Error>> {
        let config_key = format!("gitlab.{}", key.to_string().to_lowercase());
        Command::new("git")
            .arg("config")
            .arg("--global")
            .arg(config_key)
            .arg(value)
            .status()?;
        Ok(())
    }

    /// Prints the configuration property identified by 'key'.
    fn print_value(key: &ConfigureKeys) -> Result<(), Box<Error>> {
        let gitlab_config = gitlab_config::GitlabConfig::from_file()?;
        match key {
            ConfigureKeys::User => println!("{}", gitlab_config.user),
            ConfigureKeys::Host => println!("{}", gitlab_config.host),
            ConfigureKeys::SshPort => println!("{}", gitlab_config.sshport),
            ConfigureKeys::Token => println!("{}", gitlab_config.token),
            ConfigureKeys::Reset => Configure::reset(),
            ConfigureKeys::List => println!("{}", gitlab_config),
        }

        Ok(())
    }

    /// Resets the gitlab configuration, i.e. removes the whole 'gitlab' section
    /// from ~/.gitconfig.
    fn reset() {
        exit(
            Command::new("git")
                .arg("config")
                .arg("--global")
                .arg("--remove-section")
                .arg("gitlab")
                .status()
                .unwrap()
                .code()
                .unwrap(),
        );
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum ConfigureKeys {
        Host,
        SshPort,
        User,
        Token,
        Reset,
        List
    }
}
