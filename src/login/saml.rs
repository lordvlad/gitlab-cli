use structopt::StructOpt;

use super::super::gitlab_config;
use super::{check_if_token_already_set, get_password, LoginOptions};

#[derive(Debug, StructOpt)]
pub struct SamlLoginStrategy {
    #[structopt(flatten)]
    login_opts: LoginOptions,

    /// The url of the identity provider.
    idp_url: String,
}

impl SamlLoginStrategy {
    pub fn login(&self) {
        let gitlab_config = gitlab_config::GitlabConfig::from_file();
        if !self.login_opts.force {
            check_if_token_already_set(&gitlab_config);
        }

        let password = get_password(&self.login_opts);
        let user = self.login_opts.user.as_ref().unwrap_or(&gitlab_config.user);
    }
}
