use clap::{Subcommand, Args};

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCmd {
    #[clap(about = "Login to GPT-3 API")]
    /// Login to GPT-3 API
    Login(LoginArgs),
    #[clap(about = "Logout from GPT-3 API")]
    /// Logout from GPT-3 API
    Logout,
}
#[derive(Debug, Args, Clone)]
pub struct LoginArgs {
    /// API Key
    pub api_key: String,
}
