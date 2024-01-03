
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

    pub fn location(&self) -> String {
        let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
        format!("{}/config.ini", config_dir.config_dir().to_str().unwrap())
    }

    pub fn load_env(mut self) -> Self {
        if std::env::var("FLOTILLA_ENDPOINT").is_ok()
        {
            self.endpoint = std::env::var("FLOTILLA_ENDPOINT").unwrap();
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

    pub fn load_file(mut self) -> Result<Self, ini::Error> {

        let config_file = self.location();
        let contents = match ini::Ini::load_from_file(&config_file)
        {
            Ok(contents) => contents,
            Err(x) => {
                eprintln!("Error loading config file: {:?}", x);
                return Err(x);
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

        if let Some(endpoint) = contents.section(Some("api".to_owned()))
        {
            if endpoint.contains_key("endpoint")
            {
                self.endpoint = endpoint.get("endpoint").unwrap().to_owned();
            }
        } else {
            self.endpoint = String::from("https://api.jodavaho.io/hfoptpreview/v2");
        }

        Ok(self)
    }

    pub fn from_options(mut self, username:Option<String>, password:Option<String>, endpoint:Option<String>) -> Self {
        if let Some(username) = username
        {
            self.username = username.to_owned();
        }
        if let Some(password) = password.to_owned()
        {
            self.password = password.to_owned();
        }
        if let Some(endpoint) = endpoint.to_owned()
        {
            self.endpoint = endpoint.to_owned();
        }
        self
    }

    pub fn remove(self) -> Result<Self, String> {
        match std::fs::remove_file(self.location())
        {
            Ok(_) => Ok(self),
            Err(x) => Err(x.to_string())
        }
    }

    pub fn load_all(self, username_override:Option<String>, password_override:Option<String>, endpoint_override:Option<String>) -> Self {
        self.load_file().unwrap().load_env().from_options(username_override, password_override, endpoint_override)
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
            .set("username", self.username.clone())
            .set("password", self.password.clone());

        contents.with_section(Some("endpoint".to_owned()))
            .set("endpoint", self.endpoint.clone());

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
