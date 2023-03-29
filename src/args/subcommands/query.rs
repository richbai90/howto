use clap::{Args};


#[derive(Debug, Args, Clone)]
pub struct QueryArgs {
    /// The question to ask GPT-3. Note that the query is always in the context of running a shell command.
    pub query: String,
}
