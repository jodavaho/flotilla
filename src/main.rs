
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
        eprintln!("Ship: {}", ship.name);
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

fn verb_setup(matches: clap::ArgMatches) {
    if let Ok(config) = Config::new().load_file(){
        eprintln!("Overwriting existing config file.");
        let backup = format!("{}.bak", config.location());
        if let Err(x) = std::fs::copy(config.location(), backup)
        {
            eprintln!("Could not create backup of config file: {}", x);
        }
    }
    Config::new()
        .load_env()
        .add_matches(&matches)
        .save_to_default()
        .expect("Could not save config file.");
}

fn main() {
    let matches = interface::parse_cli();

    //check user verb:
    let verb = matches.subcommand();
    if verb.is_none()
    {
        eprintln!("No verb specified. Please see --help for more information.");
        return;
    }

    match verb.unwrap().0
    {
        "setup" => {
            verb_setup(matches)
        },
        "login" => {
            verb_login(matches)
        },
        "ships" => {
            verb_ships(matches)
        },
        "logout" => {
            verb_logout(matches)
        },
        _ => {
            eprintln!("Unknown verb. Please see --help for more information.");
            return;
        }
    }

}
