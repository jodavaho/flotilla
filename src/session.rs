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
    pub fn load_all(&self) -> Self {
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

        if let Ok(found) = serde_json::from_str(&contents)
        {
            return found;
        }

        Session{
            id_token: self.id_token.clone(),
            user_id: self.user_id.clone(),
            refresh_token: self.refresh_token.clone(),
            expiration_unix: self.expiration_unix.clone(),
        }


    }

    pub fn remove(self) -> Result<Self, String> {
        let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
        let config_file = config_dir.config_dir().join("session.json");
        match std::fs::remove_file(&config_file)
        {
            Ok(_) => Ok(self),
            Err(x) => Err(x.to_string()),
        }
    }

    pub fn save_to_default(&self) -> &Self{
        let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
        let config_file = config_dir.config_dir().join("session.json");
        //make sure the config directory exists
        std::fs::create_dir_all(config_dir.config_dir()).expect("Application Error: Could not create configuration directory. Please file a bug!");
        let file = std::fs::File::create(&config_file).expect("Failed to create config file");
        serde_json::to_writer_pretty(&file, self).expect("Failed to write config file");
        self
    }
}

