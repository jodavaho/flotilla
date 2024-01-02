// Purpose: Defines the CLI interface for Flotilla


use clap::{Arg, Command};

pub fn parse_cli() -> clap::ArgMatches
{
    let username_arg = Arg::new("username")
        .required(false)
        .short('u')
        .long("username")
        .value_name("USERNAME")
        .help("Manually set username (ignore config file / env)")
        .global(true)
        .action(clap::ArgAction::Set);

    let password_arg = Arg::new("password")
        .required(false)
        .short('p')
        .long("password")
        .value_name("PASSWORD")
        .help("Manually set password (ignore config file / env)")
        .global(true)
        .action(clap::ArgAction::Set);

    let endpoint_arg = Arg::new("endpoint")
        .required(false)
        .short('e')
        .long("endpoint")
        .value_name("ENDPOINT")
        .help("Manually set endpoint (ignore config file / env)")
        .global(true)
        .action(clap::ArgAction::Set);

    let config_file_location_arg = Arg::new("config")
        .required(false)
        .short('c')
        .long("config")
        .value_name("FILE")
        .help("Sets a custom config file location")
        .global(true)
        .action(clap::ArgAction::Set);

    let verbose_arg = Arg::new("verbose")
        .required(false)
        .short('v')
        .long("verbose")
        .help("Sets the verbosity level to 'loud' (all output is to stderr)")
        .global(true)
        .action(clap::ArgAction::SetTrue);

    let batch_arg = Arg::new("batch")
        .required(false)
        .short('b')
        .long("batch")
        .help("Sets the verbosity level to 'quiet', no user interaction")
        .conflicts_with("verbose")
        .global(true)
        .action(clap::ArgAction::SetTrue);

    let json_arg = Arg::new("json")
        .required(false)
        .short('j')
        .long("json")
        .help("Sets the output format to JSON")
        .action(clap::ArgAction::SetTrue)
        .global(true)
        .overrides_with("json");

    let setup_command = Command::new("setup")
        .about("Writes a new configuration file");

    let login_command = Command::new("login")
        .about("Logs in - you must have a valid username and password, visit the Hfopt website to create an account");

    let logout_command = Command::new("logout")
        .about("Logs out");

    let ships_command = Command::new("ships")
        .about("Lists all ships");

    Command::new("flotilla")
        .version("0.1.0")
        .author("Joshua Vander Hook <hello@Jodavaho.io>")
        .about("A CLI tool for managing Hfopt projects")
        .subcommand(setup_command)
        .subcommand(login_command)
        .subcommand(logout_command)
        .subcommand(ships_command)
        .arg(&config_file_location_arg)
        .arg(&username_arg)
        .arg(&password_arg)
        .arg(&endpoint_arg)
        .arg(&verbose_arg)
        .arg(&batch_arg)
        .arg(&json_arg)
        .get_matches_from(std::env::args())

}
