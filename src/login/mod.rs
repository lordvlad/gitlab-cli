use std::io::{stdin, Read};
use std::result::Result::{Err, Ok};

use dialoguer::PasswordInput;
use structopt::StructOpt;

use super::gitlab_config;

mod saml;

#[derive(Debug, StructOpt)]
pub struct Login {
    #[structopt(subcommand)]
    strategy: LoginStrategies,
}

impl Login {
    /// Logs the user in using the strategy configured in 'opt.strategy'.
    pub fn login(&self) {
        match &self.strategy {
            LoginStrategies::Saml(opt) => opt.login(),
        }
    }
}

#[derive(Debug, StructOpt)]
enum LoginStrategies {
    /// Login using SAML.
    #[structopt(name = "saml")]
    Saml(saml::SamlLoginStrategy),
}

#[derive(Debug, StructOpt)]
struct LoginOptions {
    ///  If a token has already been configured, the login command will exit
    ///  without creating a new token. Use the --force option to forcefully
    ///  create a new token.
    #[structopt(long = "force")]
    force: bool,

    /// Your username at the identity provider.
    /// Defaults to the gitlab username
    #[structopt(long = "user")]
    user: Option<String>,

    /// Your password at the identity provider. This option is
    /// INSECURE, and will leave your password in your command history.
    /// Better use --password-interactive for an interactive login, or
    /// --password-stdin to pass the password via STDIN.
    #[structopt(long = "password")]
    password: Option<String>,

    /// Pass the password via STDIN.
    #[structopt(long = "password-stdin")]
    password_stdin: bool,

    /// Ask interactively for the password. (This is the default behavior.)
    #[structopt(long = "password-interactive")]
    password_interactive: bool,
}

fn check_if_token_already_set(gitlab_config: &gitlab_config::GitlabConfig) {
    if !gitlab_config.token.is_empty() {
        panic!("There is already a token configured, refusing to running login. If you really want a new login token, use the --force flag.")
    }
}

fn get_password(opts: &LoginOptions) -> String {
    if opts.password.is_some() {
        opts.password.as_ref().unwrap().clone()
    } else if opts.password_stdin {
        let mut password: String = "".to_string();
        match stdin().read_to_string(&mut password) {
            Ok(_) => password,
            Err(e) => panic!(format!("Failed to read password from stdin {:?}", e)),
        }
    } else {
        PasswordInput::new()
            .with_prompt("Your password")
            .interact()
            .ok()
            .unwrap()
            .clone()
    }
}
