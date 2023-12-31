use clap::{Arg, Command};
use directories::ProjectDirs;

struct Config {
    username: String,
    password: String,
}

fn load_config() -> Config {

    let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
    let config_file = config_dir.config_dir().join("config.ini");
    let mut contents = match ini::Ini::load_from_file(&config_file)
    {
        Ok(contents) => contents,
        Err(_) => {
            ini::Ini::new()
        }
    };

    //does the config have a username and password?
    if let Some(user) = contents.section(Some("user".to_owned()))
    {
        if user.contains_key("username") && user.contains_key("password")
        {
            return Config {
                username: user.get("username").unwrap().to_owned(),
                password: user.get("password").unwrap().to_owned(),
            }
        }
    }

    let username = match std::env::var("FLOTILLA_USERNAME")
    {
        Ok(username) => username,
        Err(_) => {
            //prompt for username
            //save to ~/.flotilla
            //return username
            let mut username = String::new();
            println!("Please enter your username: ");
            std::io::stdin().read_line(&mut username).expect("Failed to read line");
            username
        }
    };
    contents.with_section(Some("user".to_owned()))
        .set("username", &username);
    let password = match std::env::var("FLOTILLA_PASSWORD")
    {
        Ok(password) => password,
        Err(_) => {
            //prompt for password
            //save to ~/.flotilla
            //return password
            let mut password = String::new();
            println!("Please enter your password: ");
            std::io::stdin().read_line(&mut password).expect("Failed to read line");
            password
        }
    };
    contents.with_section(Some("user".to_owned()))
        .set("password", &password);

    //make sure the config directory exists
    std::fs::create_dir_all(config_dir.config_dir()).expect("Application Error: Could not create configuration directory. Please file a bug!");
    println!("Writing to {:?}", config_file);

    match contents.write_to_file(config_file)
    {
        Ok(_) => println!("Config file written to ~/.flotilla"),
        Err(_) => eprintln!("Unable to write config file to ~/.flotilla, proceeding without saving"),
    }

    Config {
        username,
        password,
    }
}

fn main() {
    let _ = Command::new("flotilla")
        .version("0.1.0")
        .author("Joshua Vander Hook <hello@Jodavaho.io>")
        .about("A CLI tool for managing Flotilla projects")
        .arg(
            Arg::new("verbose")
            .required(false)
            .short('v')
            .long("verbose")
            .help("Sets the level of verbosity"),
        )
        .get_matches_from(std::env::args());
    let config = load_config();
    println!("Hello, {}", config.username);

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
