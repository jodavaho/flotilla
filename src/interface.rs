// Purpose: Defines the CLI interface for Flotilla

use clap::{Arg, Command};

pub fn parse_cli() -> clap::ArgMatches
{
    let username_arg = Arg::new("username")
        .required(false)
        .short('u')
        .long("username")
        .value_name("USERNAME")
        .help("Sets the username to use for Flotilla")
        .action(clap::ArgAction::Set)
        .overrides_with("username");

    let password_arg = Arg::new("password")
        .required(true)
        .short('p')
        .long("password")
        .value_name("PASSWORD")
        .help("Sets the password to use for Flotilla")
        .action(clap::ArgAction::Set)
        .overrides_with("password");

    let config_file_location_arg = Arg::new("config")
        .required(false)
        .short('c')
        .long("config")
        .value_name("FILE")
        .help("Sets a custom config file")
        .action(clap::ArgAction::Set)
        .overrides_with("config");

    let verbose_arg = Arg::new("verbose")
        .required(false)
        .short('v')
        .long("verbose")
        .help("Sets the verbosity level to 'loud' (all output is to stderr)")
        .action(clap::ArgAction::SetTrue)
        .overrides_with("verbose");

    let batch_arg = Arg::new("batch")
        .required(false)
        .short('b')
        .long("batch")
        .help("Sets the verbosity level to 'quiet', no user interaction")
        .conflicts_with("verbose")
        .action(clap::ArgAction::SetTrue)
        .overrides_with("batch");

    let json_arg = Arg::new("json")
        .required(false)
        .short('j')
        .long("json")
        .help("Sets the output format to JSON")
        .action(clap::ArgAction::SetTrue)
        .overrides_with("json");

    let setup_command = Command::new("setup")
        .about("Writes a new configuration file")
        .arg(username_arg.clone())
        .arg(password_arg.clone())
        .arg(config_file_location_arg.clone());

    let login_command = Command::new("login")
        .about("Logs in - you must have a valid username and password, visit the Hfopt website to create an account")
        .arg(username_arg.clone())
        .arg(password_arg.clone());

    let logout_command = Command::new("logout")
        .about("Logs out");

    let ships_command = Command::new("ships")
        .about("Lists all ships")
        .arg(json_arg.clone())
        .arg(verbose_arg.clone())
    .arg(batch_arg.clone());

    Command::new("flotilla")
        .version("0.1.0")
        .author("Joshua Vander Hook <hello@Jodavaho.io>")
        .about("A CLI tool for managing Flotilla projects")
        .subcommand(setup_command)
        .subcommand(login_command)
        .subcommand(logout_command)
        .subcommand(ships_command)
        .get_matches_from(std::env::args())

}
