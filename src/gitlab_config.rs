use std::collections::HashMap;
use std::fmt;

use dirs::home_dir;
use ini::Ini;
use whoami::username;

#[derive(Debug)]
pub struct GitlabConfig {
    pub host: String,
    pub sshport: u16,
    pub user: String,
    pub token: String,
}

impl std::fmt::Display for GitlabConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "host: {}\nsshport: {}\nuser: {}\ntoken: {}",
            self.host, self.sshport, self.user, self.token
        )
    }
}

impl GitlabConfig {
    pub fn api(&self) -> String {
        return format!("https://{}/api/v4", self.host);
    }

    pub fn clone_base(&self) -> String {
        return format!("ssh://git@{}:{}/", self.host, self.sshport);
    }

    pub fn from_file() -> GitlabConfig {
        let empty_config: HashMap<String, String> = HashMap::new();
        let git_config_path = format!("{}/.gitconfig", home_dir().unwrap().display());
        let git_config = Ini::load_from_file(&git_config_path).unwrap();
        let props = git_config
            .section(Some("gitlab".to_owned()))
            .unwrap_or(&empty_config);

        GitlabConfig {
            host: props.get("host").unwrap_or(&"".to_owned()).to_owned(),
            sshport: props
                .get("sshport")
                .unwrap_or(&"22".to_owned())
                .to_owned()
                .parse()
                .unwrap(),
            user: props
                .get("user")
                .unwrap_or(&username().to_owned())
                .to_owned(),
            token: props.get("token").unwrap_or(&"".to_owned()).to_owned(),
        }
    }
}
