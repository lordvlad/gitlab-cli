use structopt::StructOpt;

mod clone;
mod configure;
mod gitlab_config;
mod login;
mod publish;
mod status;

#[derive(Debug, StructOpt)]
struct GitlabCli {
    /// Show additional debug output
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Override the default output formatting
    #[structopt(long = "format", short = "f")]
    format: Option<String>,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    /// Check if there is a newer version of gitlab available and install it.
    #[structopt(name = "update")]
    Update,

    /// Initialize a new project interactively. This makes use of templates and
    /// variables to stamp out a new project and publish it on gitlab.
    #[structopt(name = "init")]
    Init,

    /// Print completion function for bash to stdout.
    #[structopt(name = "completion")]
    Completion,

    /// Clone a project from gitlab, essentially doing `git clone` but with a couple of
    /// extra features.
    #[structopt(name = "clone")]
    GitClone(clone::GitClone),

    /// Change or display gitlab configuration.
    #[structopt(name = "configure")]
    Configure(configure::Configure),

    /// Show the status of the CI pipelines.
    #[structopt(name = "status")]
    Status(status::Status),

    /// Publish a project to gitlab.
    #[structopt(name = "publish")]
    Publish(publish::Publish),

    /// Log into gitlab and obtain an API token for further access.
    #[structopt(name = "login")]
    Login(login::Login),
}

fn main() {
    let args = GitlabCli::from_args();

    match args.cmd {
        Cmd::Configure(opt) => opt.configure(),
        Cmd::Publish(opt) => opt.publish(),
        Cmd::Status(opt) => opt.status(),
        Cmd::GitClone(opt) => opt.clone(),
        Cmd::Login(opt) => opt.login(),
        Cmd::Completion => completion(),
        Cmd::Init => init(),
        Cmd::Update => update(),
    }
}

fn completion() {
    unimplemented!();
}
fn init() {
    unimplemented!();
}
fn update() {
    unimplemented!();
}
