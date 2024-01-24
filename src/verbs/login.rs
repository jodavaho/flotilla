use crate::config::Config;
use crate::api;
pub fn exec(username: Option<String>, password: Option<String>, endpoint: Option<String>) -> Result<(), String> 
{

    let config = Config::new().load_all(username, password, endpoint);
    match api::login(&config) {
        Ok(sess) => {
            println!("Logged in until {}", 
                     chrono::NaiveDateTime::from_timestamp_opt(sess.expiration_unix, 0).unwrap().to_string());
            Ok(())
        },
        Err(e) => {
            Err(format!("Could not log in: {}", e))
        }
    }
}

