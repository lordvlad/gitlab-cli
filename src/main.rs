use std::cell::RefCell;
use std::error::Error;
use structopt::StructOpt;

mod clone;
mod completion;
mod configure;
mod gitlab_config;
mod login;
mod publish;
mod status;

thread_local!(pub static DEBUG: RefCell<bool> = RefCell::new(false));
thread_local!(pub static FORMAT: RefCell<Option<String>> = RefCell::new(None));

fn get_format() -> Option<String> {
    FORMAT.with(|format| format.borrow().clone())
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        crate::DEBUG.with(|debug| {
            if debug.borrow().clone() {
                eprintln!("[DEBUG] {}", format!($($arg)*))
            }
        })
    })
}

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => ({
        eprintln!("[FAIL] {}", format!($($arg)*));
        std::process::exit(1);
    })

}

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
    Completion(completion::Completion),

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = GitlabCli::from_args();

    DEBUG.with(|debug| {
        debug.replace(args.debug);
    });
    FORMAT.with(|format| {
        format.replace(args.format.clone());
    });

    match args.cmd {
        Cmd::Configure(opt) => opt.configure(),
        Cmd::Publish(opt) => opt.publish(),
        Cmd::Status(opt) => opt.status(),
        Cmd::GitClone(opt) => opt.git_clone(),
        Cmd::Login(opt) => opt.login(),
        Cmd::Completion(opt) => opt.completion(),
        Cmd::Init => init(),
        Cmd::Update => update(),
    }
}

fn init() -> Result<(), Box<dyn Error>> {
    unimplemented!();
}
fn update() -> Result<(), Box<dyn Error>> {
    unimplemented!();
}
