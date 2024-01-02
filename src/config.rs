
use directories::ProjectDirs;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub endpoint: String,
}

pub fn load_all() -> Config {

    let mut retval:Config = Config {
        username: String::from(""),
        password: String::from(""),
        endpoint: String::from(""),
    };

    let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
    let config_file = config_dir.config_dir().join("config.ini");
    let contents = match ini::Ini::load_from_file(&config_file)
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
            retval.username = user.get("username").unwrap().to_owned();
            retval.password = user.get("password").unwrap().to_owned();
        }
    }

    if let Some(api) = contents.section(Some("api".to_owned()))
    {
        if api.contains_key("endpoint")
        {
            retval.endpoint = api.get("endpoint").unwrap().to_owned();
        }
    }

    if std::env::var("FLOTILLA_ENDPOINT").is_ok()
    {
        retval.endpoint = std::env::var("FLOTILLA_ENDPOINT").unwrap();
    }
    else if let Some(endpoint) = contents.section(Some("api".to_owned()))
    {
        if endpoint.contains_key("endpoint")
        {
            retval.endpoint = endpoint.get("endpoint").unwrap().to_owned();
        }
    } else {
        retval.endpoint = String::from("https://api.jodavaho.io/hfoptpreview/v2");
    }


    if std::env::var("FLOTILLA_USERNAME").is_ok()
    {
        retval.username = std::env::var("FLOTILLA_USERNAME").unwrap();
    }

    if std::env::var("FLOTILLA_PASSWORD").is_ok()
    {
        retval.password = std::env::var("FLOTILLA_PASSWORD").unwrap();
    }
    retval
}

pub fn save_file(config: Config) {
    let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
    let config_file = config_dir.config_dir().join("config.ini");
    let mut contents = match ini::Ini::load_from_file(&config_file)
    {
        Ok(contents) => contents,
        Err(_) => {
            ini::Ini::new()
        }
    };

    contents.with_section(Some("user".to_owned()))
        .set("username", config.username)
        .set("password", config.password);

    contents.with_section(Some("endpoint".to_owned()))
        .set("endpoint", config.endpoint);

    //make sure the config directory exists
    std::fs::create_dir_all(config_dir.config_dir()).expect("Application Error: Could not create configuration directory. Please file a bug!");
    println!("Writing to {:?}", config_file);

    contents.write_to_file(&config_file).expect("Failed to write config file");
}
