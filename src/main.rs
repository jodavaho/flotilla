
mod interface;
mod config;
use config::Config;
mod session;
mod api;

fn verb_ships(matches: clap::ArgMatches)
{
    let config = Config::new().load_all().add_matches(matches.clone());
    eprintln!("Hello, {}", config.username);
    eprintln!("Loading flotilla from {}", config.endpoint);
    eprintln!("config: API endpoint:{}",config.endpoint);
    let session = api::login(&config).expect("Could not log in");
    let app = api::Flotilla { config:*config, session:session };
    let user_data = app.get_user_data().expect("Could not get user data");
    eprintln!("User data: {:?}", user_data);
}

fn verb_login(matches: clap::ArgMatches)
{
    let config = Config::new().load_all().add_matches(matches.clone());
    api::login(&config).expect("Could not log in").save_to_default();
    eprintln!("Hello, {}, login session saved.", config.username);
}

fn verb_setup(matches: clap::ArgMatches) {
    let config = Config::new().add_matches(matches.clone()).save_to_default();
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
        _ => {
            eprintln!("Unknown verb. Please see --help for more information.");
            return;
        }
    }

}
