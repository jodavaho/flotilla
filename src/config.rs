
use directories::ProjectDirs;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub endpoint: String,
}



impl Config {
    #[allow(dead_code)]
    pub fn new() -> Config {
        Config {
            username: String::from(""),
            password: String::from(""),
            endpoint: String::from("https://api.jodavaho.io/hfopt/v2"),
        }
    }

    #[allow(dead_code)]
    pub fn add_matches(&self, matches: clap::ArgMatches) -> &Config {
        let mut config = Config::new();
        if let Some(username) = matches.get_one::<String>("username") {
            self.username = username.clone();
        }
        if let Some(password) = matches.get_one::<String>("password") {
            self.password = password.clone();
        }
        if let Some(endpoint) = matches.get_one::<String>("endpoint") {
            self.endpoint = endpoint.clone();
        }
        self
    }

    pub fn load_all(&self) -> &Config {

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
                self.username = user.get("username").unwrap().to_owned();
                self.password = user.get("password").unwrap().to_owned();
            }
        }

        if let Some(api) = contents.section(Some("api".to_owned()))
        {
            if api.contains_key("endpoint")
            {
                self.endpoint = api.get("endpoint").unwrap().to_owned();
            }
        }

        if std::env::var("FLOTILLA_ENDPOINT").is_ok()
        {
            self.endpoint = std::env::var("FLOTILLA_ENDPOINT").unwrap();
        }
        else if let Some(endpoint) = contents.section(Some("api".to_owned()))
        {
            if endpoint.contains_key("endpoint")
            {
                self.endpoint = endpoint.get("endpoint").unwrap().to_owned();
            }
        } else {
            self.endpoint = String::from("https://api.jodavaho.io/hfoptpreview/v2");
        }


        if std::env::var("FLOTILLA_USERNAME").is_ok()
        {
            self.username = std::env::var("FLOTILLA_USERNAME").unwrap();
        }

        if std::env::var("FLOTILLA_PASSWORD").is_ok()
        {
            self.password = std::env::var("FLOTILLA_PASSWORD").unwrap();
        }
        self
    }

    pub fn save_to_default(&self, ) -> Result<&Config, std::io::Error>{
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
            .set("username", self.username)
            .set("password", self.password);

        contents.with_section(Some("endpoint".to_owned()))
            .set("endpoint", self.endpoint);

        //make sure the config directory exists
        std::fs::create_dir_all(config_dir.config_dir()).expect("Application Error: Could not create configuration directory. Please file a bug!");
        println!("Writing to {:?}", config_file);

        match contents.write_to_file(&config_file)
        {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }
}
