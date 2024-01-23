
mod api;
mod config;
mod interface;
mod session;
mod verbs;

use interface::SubCommand::*;
use verbs::verify;
use verbs::setup;
use verbs::login;
use verbs::logout;
use verbs::list;
use verbs::get;
use verbs::fetch;
use verbs::edit;

fn main() 
{
    let cli:interface::Cli = argp::parse_args_or_exit(argp::DEFAULT);

    match cli.subcommand
    {
        Verify(options) => verify::exec(options.file),
        Setup(options) => setup::exec(options.username, options.password, options.endpoint),
        Login(options) => login::exec(options.username, options.password, options.endpoint),
        Logout(_) => logout::exec(),
        Get(options) => get::exec(options.id, options.public),
        List(options) => list::exec(options.what),
        Fetch(_) => fetch::exec(),
        Edit(options) => edit::exec(options.id, options.operation),
    }

}
