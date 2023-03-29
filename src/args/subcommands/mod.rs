pub mod auth;
pub mod query;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    #[clap(subcommand)]
    Auth(auth::AuthCmd),
    #[clap(name = "query")]
    Query(query::QueryArgs),
}
