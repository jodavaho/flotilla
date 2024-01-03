
mod api;
mod config;
mod interface;
mod session;
use clap::Parser;
use config::Config;
//use interface::Cli;
use interface::SubCommand::*;
use session::Session;

fn verb_login(username: Option<String>, password: Option<String>, endpoint: Option<String>)
{

    let config = Config::new().load_all(username, password, endpoint);
    api::login(&config)
        .expect("Could not log in")
        .save_to_default();
}

fn verb_logout()
{
    match config::Config::new().load_file().expect("Could not load config file").remove()
    {
        Ok(_) => eprintln!("Config removed"),
        Err(x) => eprintln!("Could not remove config: {}", x),
    }
    match Session::new().remove()
    {
        Ok(_) => eprintln!("Sesison removed"),
        Err(x) => eprintln!("Could not remove session: {}", x),
    }
}

fn verb_setup(username: Option<String>, password: Option<String>, endpoint: Option<String>) {
    if let Ok(config) = Config::new().load_file(){
        eprintln!("Overwriting existing config file.");
        let backup = format!("{}.bak", config.location());
        if let Err(x) = std::fs::copy(config.location(), backup)
        {
            eprintln!("Could not back up config file: {}", x);
        }
    }
    let cfg = Config::new().load_env().from_options(username, password, endpoint);
    if let Err(x) = cfg.save_to_default()
    {
        eprintln!("Could not save config file: {}", x);
    } 
    eprintln!("Config file saved to {}", cfg.location());
}

fn verb_verify(file: std::path::PathBuf)
{
    eprintln!("Verifying {}", file.display());
}


fn main() 
{
    let cli = interface::Cli::parse();
    match cli.subcommand
    {
        Setup{username, password, endpoint} => verb_setup(username, password, endpoint),
        Login{username, password, endpoint} => verb_login(username, password, endpoint),
        Logout => verb_logout(),
        Verify { file } => verb_verify(file),
        List { what } => eprintln!("List {:?}", what ),
        Get { id } => eprintln!("Get {}", id),

    }

}
