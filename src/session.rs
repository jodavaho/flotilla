use chrono::Utc;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Session {
    pub id_token: String,
    pub user_id: String,
    pub refresh_token: String,
    pub expiration_unix: i64,
}

impl Session{

    pub fn new() -> Session {
        Session {
            id_token: String::from(""),
            user_id: String::from(""),
            refresh_token: String::from(""),
            expiration_unix: Utc::now().timestamp()-1,
        }
    }

    pub fn expired(&self) -> bool {
        let now = Utc::now().timestamp();
        if now >= self.expiration_unix {
            return true;
        }
        return false;
    }
    pub fn load_all(& mut self) -> &Self {
        let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
        let config_file = config_dir.config_dir().join("session.json");

        //load raw config file contents into a string for serde
        let contents = match std::fs::read_to_string(&config_file)
        {
            Ok(contents) => contents,
            Err(_) => {
                String::from("")
            }
        };

        let found = serde_json::from_str(&contents).unwrap_or(Session {
            id_token: String::from(""),
            user_id: String::from(""),
            refresh_token: String::from(""),
            expiration_unix: Utc::now().timestamp()-1,
        });

        self.id_token = found.id_token;
        self.user_id = found.user_id;
        self.refresh_token = found.refresh_token;
        self.expiration_unix = found.expiration_unix;
        self
    }

    pub fn save_to_default(&self) -> &Self{
        let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
        let config_file = config_dir.config_dir().join("session.json");
        //make sure the config directory exists
        std::fs::create_dir_all(config_dir.config_dir()).expect("Application Error: Could not create configuration directory. Please file a bug!");
        println!("Writing to {:?}", config_file);
        let file = std::fs::File::create(&config_file).expect("Failed to create config file");
        serde_json::to_writer_pretty(&file, self).expect("Failed to write config file");
        self
    }
}

