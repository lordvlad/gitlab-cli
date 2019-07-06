use indoc::indoc;
use std::error::Error;
use std::time::Duration;

use reqwest::ClientBuilder;
use scraper::{Html, Selector};
use structopt::StructOpt;

// use super::super::{gl_assert, debug, gitlab_config};
use super::super::{debug, gitlab_config};
use super::{check_if_token_already_set, get_password, LoginOptions};

#[derive(Debug, StructOpt)]
pub struct SamlLoginStrategy {
    #[structopt(flatten)]
    login_opts: LoginOptions,

    /// The url of the identity provider.
    idp_url: String,
}

impl SamlLoginStrategy {
    pub fn login(&self) -> Result<(), Box<Error>> {
        let gitlab_config = gitlab_config::GitlabConfig::from_file()?;

        if !self.login_opts.force {
            check_if_token_already_set(&gitlab_config);
        }

        let password = get_password(&self.login_opts)?;
        let user = self.login_opts.user.as_ref().unwrap_or(&gitlab_config.user);

        let timeout = Duration::new(5, 0);
        let client = ClientBuilder::new().timeout(timeout).build()?;

        let mut response = client.get(&self.idp_url).send()?;

        assert!(response.status().is_success(), indoc!("
            Failed to load the identity provider url {}. Please double check the idp-url option and try again."), self.idp_url
        );

        let mut fragment = Html::parse_document(&response.text().unwrap());

        let post_url = fragment
            .select(&Selector::parse("form").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("action")
            .unwrap();

        debug!(
            "Logging in at url {} with user {}  and password {}.",
            post_url, user, password
        );

        let mut response = client
            .post(post_url)
            .form(&[("username", &user), ("password", &&password)])
            .send()
            .unwrap();

        assert!(response.status().is_success(), indoc(
            "Failed to log in with username {} and password {}, server returned status code {}."
            ),
            user, password, response.status() );

        Ok(())
    }
}
