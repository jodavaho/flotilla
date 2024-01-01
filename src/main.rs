use clap::{Arg, Command};
use chrono::Utc;
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
    eprintln!("config: API endpoint:{}",config.endpoint);

    let client = reqwest::blocking::Client::new();

    let json_body = serde_json::json!({
        "email": config.username,
        "password": config.password,
    });

    let mut session = session::load_session();
    eprintln!("Session: {:?}",session);
    if session.expired()
    {
        eprintln!("Session expired, logging in");
        let auth_url = format!("{}/user/quick_login", config.endpoint);
        eprintln!("Auth URL: {}",auth_url);
        let res = match client
            .post(auth_url)
            .header("Content-Type", "application/json")
            .body(json_body.to_string())
            .send()
            {
                Ok(res) => res,
                Err(_) => {
                    eprintln!("Error: Could not connect to server");
                    return;
                }
            };

        if res.status().is_success()
        {
            eprintln!("Login successful");
            let json: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
            session.id_token = json["AuthenticationResult"]["IdToken"].to_string();
            session.user_id = config.username.clone();
            session.refresh_token = json["AuthenticationResult"]["RefreshToken"].to_string();
            session.expiration_unix = 
                json["AuthenticationResult"]["ExpiresIn"].as_i64().unwrap()+Utc::now().timestamp();
            session::save_session(&session);
        } else {
            eprintln!("Error: Could not log in");
            return;
        }
    }

    eprintln!("Session: {:?}",session);







}
