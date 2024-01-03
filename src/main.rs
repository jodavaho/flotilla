
mod interface;
mod config;
use config::Config;
mod session;
mod api;

fn verb_ships(matches: clap::ArgMatches)
{
    let config = Config::new().load_all(&matches);
    eprintln!("Hello, {}", config.username);
    eprintln!("Loading flotilla from {}", config.endpoint);
    eprintln!("config: API endpoint:{}",config.endpoint);
    let session = api::login(&config).expect("Could not log in");
    dbg!("Session: {:?}",&session);
    let app = api::Flotilla::new(&config, &session);
    let user_data = app.get_user_data().expect("Could not get user data");
    for ship in user_data.0
    {
        eprintln!("{} {} {} {} {} {}", ship.short_id, ship.name, ship.file_name, ship.downloads, ship.uploaded, ship.download_url);
    }
}

fn verb_login(matches: clap::ArgMatches)
{
    let config = Config::new()
        .load_all(&matches);
    api::login(&config)
        .expect("Could not log in")
        .save_to_default();
}

fn verb_logout(_matches: clap::ArgMatches)
{
    if let Err(x) = std::fs::remove_file(Config::new().location())
    {
        eprintln!("Could not remove config file: {}", x);
    }
}

//fn verb_setup(matches: clap::ArgMatches) {
fn verb_setup(username: Option<String>, password: Option<String>, endpoint: Option<String>) {
    if let Ok(config) = Config::new().load_file(){
        eprintln!("Overwriting existing config file.");
        let backup = format!("{}.bak", config.location());
        if let Err(x) = std::fs::copy(config.location(), backup)
        {
            eprintln!("Could not back up config file: {}", x);
        }
    }
    let mut cfg = Config::new().load_env();
    if let Some(username) = username
    {
        cfg.username = username.to_owned();
    }
    if let Some(password) = password.to_owned()
    {
        cfg.password = password.to_owned();
    }
    if let Some(endpoint) = endpoint.to_owned()
    {
        cfg.endpoint = endpoint.to_owned();
    }
    if let Err(x) = cfg.save_to_default()
    {
        eprintln!("Could not save config file: {}", x);
    } 
    eprintln!("Config file saved to {}", cfg.location());
}

use clap::Parser;

fn main() 
{
    let cli = interface::Cli::parse();
}
