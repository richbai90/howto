mod args;
mod openai;

use clap::Parser;

fn main() {
    let mut client = openai::client::Client::new();
    let args = args::HowToArgs::parse();
    match args.cmd {
        args::SubCommands::Auth(cmd) => match cmd {
            // If cmd is Login, then we can access the fields of the Login struct
            args::AuthCmd::Login(args) => {
                client.login(Some(args.api_key));
            }
            // If cmd is logout there are no fields to access
            args::AuthCmd::Logout => {
                println!("Logout");
            }
        },
        args::SubCommands::Query(args) => {
            client.login(None);
            let res = client.query(&args.query);
            println!(
                "{}",
                res.unwrap_or(String::from("There was a problem with the request"))
            );
        }
    }
}
