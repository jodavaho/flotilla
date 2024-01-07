use crate::config::Config;
use crate::session::Session;
use crate::api;

pub fn exec(what: Option<String>)
{

    let config = Config::new().load_env().load_file().expect("Application Error: Could not load configuration file. Please file a bug!");
    let session = Session::new().load_all();
    if session.expired()
    {
        println!("Session expired. Please login.");
        return;
    }

    let flotilla = api::Flotilla::new(&config, &session);
    if what.is_none()
    {
        match flotilla.get_user_data()
        {
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            },
            Ok(user_data) => {
                let user_data_json = serde_json::to_string_pretty(&user_data).expect("Application Error: Could not serialize user data. Please file a bug!");
                println!("{}", &user_data_json);
            },
        }
        return;
    }

    match what.unwrap().as_str()
    {
        "ships" => {
            match flotilla.get_user_data()
            {
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                },
                Ok(user_data) => {
                    let user_data_json = serde_json::to_string_pretty(&user_data.ships).expect("Application Error: Could not serialize user data. Please file a bug!");
                    println!("{}", &user_data_json);
                },
            }
        },
        "collections" => {
            match flotilla.get_user_data()
            {
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                },
                Ok(user_data) => {
                    let user_data_json = serde_json::to_string_pretty(&user_data.collections).expect("Application Error: Could not serialize user data. Please file a bug!");
                    println!("{}", &user_data_json);
                },
            }
        },
        _ => {
            eprintln!("Error: Unknown argument");
            return;
        }
    }


}
