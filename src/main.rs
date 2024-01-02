use clap::{Arg, Command};

mod config;
mod session;
mod api;

fn main() {
    let matches = Command::new("flotilla")
        .version("0.1.0")
        .author("Joshua Vander Hook <hello@Jodavaho.io>")
        .about("A CLI tool for managing Flotilla projects")
        .arg(
            Arg::new("verbose")
            .required(false)
            .short('v')
            .long("verbose")
            .help("Sets the level of verbosity")
            .action(clap::ArgAction::SetTrue)
            )
        .arg(
            Arg::new("setup")
            .required(false)
            .long("setup")
            .help("writes new configuration file")
            .action(clap::ArgAction::SetTrue)
            )
        .get_matches_from(std::env::args());

    if matches.get_flag("setup")
    {
        let config = config::load_all();
        config::save_file(config);
        return;
    }


    let config = config::load_all();
    eprintln!("Hello, {}", config.username);
    eprintln!("Loading flotilla from {}", config.endpoint);
    eprintln!("config: API endpoint:{}",config.endpoint);

    let session = api::login(&config).expect("Could not log in");

    let app = api::Flotilla { config, session };
    let user_data = app.get_user_data().expect("Could not get user data");
    eprintln!("User data: {:?}", user_data);
}
