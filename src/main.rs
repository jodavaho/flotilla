use clap::{Arg, Command};
mod config;
mod session;

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

    let client = reqwest::blocking::Client::new();

    let json_body = serde_json::json!({
        "email": config.username,
        "password": config.password,
    });

    println!("{}",json_body.to_string());
    let res = client
        //.post("https://api.jodavaho.io/hfoptpreview/v2/user/quick_login")
        .post("http://localhost:8000/hfoptpreview/v2/user/quick_login")
        .header("Content-Type", "application/json")
        .body(json_body.to_string())
        .send()
        .unwrap();
    println!("{}",res.text().unwrap());

}
