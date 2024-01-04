
mod api;
mod config;
mod interface;
mod session;
mod verbs;
use clap::Parser;
use interface::SubCommand::*;
use verbs::verify;
use verbs::setup;
use verbs::login;
use verbs::logout;

fn main() 
{
    let cli = interface::Cli::parse();
    match cli.subcommand
    {
        Setup  { username, password, endpoint } => setup::exec(username, password, endpoint),
        Login  { username, password, endpoint } => login::exec(username, password, endpoint),
        Logout { }  => logout::exec(),
        Verify { file } => verify::exec(file),
        List   { what } => eprintln!("List {:?}", what ),
        Get    { id } => eprintln!("Get {}", id),
        Fetch  { }  => eprintln!("Fetch"),
    }

}
