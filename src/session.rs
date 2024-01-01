use chrono::{DateTime, Utc};
use directories::ProjectDirs;

pub struct Session {
    pub id_token: String,
    pub user_id: String,
    pub refresh_token: String,
    pub expiration: DateTime<Utc>,
}

pub fn load_session() -> Session {
    let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
    let config_file = config_dir.config_dir().join("session.ini");
    let contents = match ini::Ini::load_from_file(&config_file)
    {
        Ok(contents) => contents,
        Err(_) => {
            ini::Ini::new()
        }
    };

    let mut retval:Session = Session {
        id_token: String::from(""),
        user_id: String::from(""),
        refresh_token: String::from(""),
        expiration: Utc::now(),
    };

    if let Some(session) = contents.section(Some("session".to_owned()))
    {
        if session.contains_key("id_token") && session.contains_key("user_id") && session.contains_key("refresh_token") && session.contains_key("expiration")
        {
            retval.id_token = session.get("id_token").unwrap().to_owned();
            retval.user_id = session.get("user_id").unwrap().to_owned();
            retval.refresh_token = session.get("refresh_token").unwrap().to_owned();
            retval.expiration = DateTime::parse_from_rfc3339(session.get("expiration").unwrap()).unwrap().with_timezone(&Utc);
        }
    }

    retval
}

pub fn save_session(session: Session) {
    let config_dir = ProjectDirs::from("io", "Jodavaho", "Flotilla").expect("Application Error: Could not load configuration directory. Please file a bug!");
    let config_file = config_dir.config_dir().join("session.ini");
    let mut contents = match ini::Ini::load_from_file(&config_file)
    {
        Ok(contents) => contents,
        Err(_) => {
            ini::Ini::new()
        }
    };

    contents.with_section(Some("session".to_owned()))
        .set("id_token", session.id_token)
        .set("user_id", session.user_id)
        .set("refresh_token", session.refresh_token)
        .set("expiration", session.expiration.to_rfc3339());

    //make sure the config directory exists
    std::fs::create_dir_all(config_dir.config_dir()).expect("Application Error: Could not create configuration directory. Please file a bug!");
    println!("Writing to {:?}", config_file);

    contents.write_to_file(&config_file).expect("Failed to write config file");
}
