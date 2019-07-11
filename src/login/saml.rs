use std::error::Error;
use std::time::Duration;

use reqwest::{Client, ClientBuilder, RedirectPolicy};
use reqwest::header::CONTENT_TYPE;
use scraper::{Html, Selector};
use structopt::StructOpt;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use super::super::{debug, gitlab_config};
use super::super::configure::{Configure, ConfigureKeys};
use super::{check_if_token_already_set, get_password, LoginOptions};

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

#[derive(Debug, StructOpt)]
pub struct SamlLoginStrategy {
    #[structopt(flatten)]
    login_opts: LoginOptions,

    /// The url of the identity provider.
    idp_url: String,
}

impl SamlLoginStrategy {
    pub fn login(&self) -> Result<(), Box<dyn Error>> {
        let gitlab_config = gitlab_config::GitlabConfig::from_file()?; 
        let gitlab_token_url=format!("{}/../../profile/personal_access_tokens", gitlab_config.api()); 

        if !self.login_opts.force {
            check_if_token_already_set(&gitlab_config);
        }

        let password = get_password(&self.login_opts)?;
        let user = self.login_opts.user.as_ref().unwrap_or(&gitlab_config.user);

        let timeout = Duration::new(5, 0);
        let client: Client = ClientBuilder::new()
            .timeout(timeout)
            .redirect(RedirectPolicy::custom(|attempt| {
                if attempt.previous().len() > 5 {
                    attempt.too_many_redirects()
                } else {
                    debug!("Following redirect to {}", attempt.url());
                    attempt.follow()
                }
            }))
            .cookie_store(true)
            .build()?;

        let mut login_form_response = client.get(&self.idp_url).send()?;

        assert!(login_form_response.status().is_success(), "Failed to load the identity provider url {}. Please double check the idp-url option and try again.", self.idp_url);

        let login_form = Html::parse_document(&login_form_response.text()?);

        let form_selector = Selector::parse("form").unwrap();

        let post_url = login_form
            .select(&form_selector)
            .next()
            .expect("Server response is missing a form element.")
            .value()
            .attr("action")
            .expect("Form in server response is missing an action attribute.");

        debug!(
            "Logging in at url {} with user {} and password {}.",
            post_url, user, password
        );

        let mut saml_response = client
            .post(post_url)
            .form(&[("username", &user), ("password", &&password)])
            .send()?;

        assert!(saml_response.status().is_success(), "Failed to log in with username {} and password {}, server returned status code {}. \n{}", user, password, saml_response.status(), saml_response.text()?);

        let saml_response_form = Html::parse_document(&saml_response.text().unwrap()); 
        let callback_url = saml_response_form
            .select(&form_selector)
            .next()
            .expect("Server response is missing a form element.")
            .value()
            .attr("action")
            .expect("Form in server response is missing an action attribute");

        let assertion = saml_response_form
            .select(&Selector::parse("input").unwrap())
            .next()
            .expect("Server response is missing an input element.")
            .value()
            .attr("value")
            .expect("Input element in server response is missing a value attribute");

        assert!(
            callback_url.len() > 0,
            "Server response contained a form element with an empty action attribute."
        );
        assert!(
            assertion.len() > 0,
            "Server response contained a input element with an empty value attribute."
        );

        let assertion_chunk: String = assertion.to_string().chars().take(10).collect();
        debug!(
            "Found callback url {} and assertion {}...",
            callback_url, assertion_chunk
        );

        client
            .post(callback_url)
            .form(&[("SAMLResponse", &assertion)])
            .send()?;


        let mut token_page_response = client
            .get(&gitlab_token_url)
            .send()?;

        assert!(token_page_response.status().is_success(), "Failed to fetch token page");
        let token_form = Html::parse_document(&token_page_response.text().unwrap()); 

        debug!( "Found token form");

        let csrf_input_selector = Selector::parse("[name=\"authenticity_token\"]").unwrap();
        let csrf_token = token_form
            .select(&csrf_input_selector)
            .next()
            .expect("Server response is missing an authenticity token form element")
            .value()
            .attr("value")
            .expect("Authenticity token form element in server response is empty");

        debug!("Found authenticity token {}.", csrf_token);

        assert!(
            csrf_token.len() > 0,
            "Server response contained an empty authenticity token."
        );

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .collect();

        let token_name = ["gitlab-cli-", &rand_string[0..7]].concat();

        debug!("Requesting new token named {}.", token_name);

        let mut new_token_response = client
            .post(&gitlab_token_url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(format!("authenticity_token={}&personal_access_token[name]={}&personal_access_token[scopes][]=api&personal_access_token[expires_at]=", &csrf_token, &token_name.as_str()))
            .send()?;

        assert!(new_token_response.status().is_success(), "Failed to create new token, server returned status code {}. \n{}", new_token_response.status(), new_token_response.text()?);

        let new_token_response_form = Html::parse_document(&new_token_response.text().unwrap()); 
        let token_selector = Selector::parse("#created-personal-access-token").unwrap();
        let token = new_token_response_form
            .select(&token_selector)
            .next()
            .expect("Server response is missing a created token")
            .value()
            .attr("value")
            .expect("Token form element in server response is  empty");

        Configure::set_value(&ConfigureKeys::Token, &token)?;
        debug!("Stored new api token {}.", token);

        Ok(())
    }
}
