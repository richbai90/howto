mod subcommands;
use clap::{Parser};

pub use subcommands::SubCommands;
pub use subcommands::auth::{AuthCmd, LoginArgs};
pub use subcommands::query::{QueryArgs};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct HowToArgs {
    #[clap(subcommand)]
    pub cmd: SubCommands,
}
