
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
    match argp::parse_args_or_exit::<interface::Cli>(argp::DEFAULT)
        .subcommand
    {
        Verify(options) => verify::exec(options.file),
        Setup(options) => setup::exec(options.username, options.password, options.endpoint),
        Login(options) => login::exec(options.username, options.password, options.endpoint),
        Logout(_) => logout::exec(),
        Get(options) => get::exec(options.ids, options.public),
        List(options) => list::exec(options.what),
        Fetch(_) => fetch::exec(),
        Edit(options) => edit::exec(options.id, options.operation),
    }
    .unwrap_or_else(|e| eprintln!("{}", e));

}
