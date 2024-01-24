use crate::config::Config;
use crate::session::Session;
use crate::api;

pub fn exec(what: Option<String>) -> Result<(), String> {

    let config = Config::new()
        .load_env()
        .load_file()
        .map_err(|e| 
                 format!("Application Error: Could not load configuration file. Please file a bug! {}", e))?;

    let session = Session::new().load_all();
    if session.expired()
    {
        return Err("Session expired. Please login.".to_string());
    }

    let flotilla = api::Flotilla::new(&config, &session);
    let user_data = flotilla.get_user_data().map_err(|e| format!("Error: {}", e))?;

    return match what.as_deref()
    {
        None => {
            let user_data_json = serde_json::to_string_pretty(&user_data).expect("Application Error: Could not serialize user data. Please file a bug!");
            println!("{}", &user_data_json);
            Ok(())
        },
        Some("ships") => {
            let user_data_json = serde_json::to_string_pretty(&user_data.ships).expect("Application Error: Could not serialize user data. Please file a bug!");
            println!("{}", &user_data_json);
            Ok(())
        },
        Some("collections") => {
            let user_data_json = serde_json::to_string_pretty(&user_data.collections).expect("Application Error: Could not serialize user data. Please file a bug!");
            println!("{}", &user_data_json);
            Ok(())
        },
        _ => {
            Err("Invalid argument. Please use 'ships' or 'collections'".to_string())
        }
    }

}
