use crate::config::Config;
use crate::api;
pub fn exec(username: Option<String>, password: Option<String>, endpoint: Option<String>)
{

    let config = Config::new().load_all(username, password, endpoint);
    let sess = api::login(&config)
        .expect("Could not log in")
        .save_to_default().expiration_unix;
    println!("Logged in until {}", 
             chrono::NaiveDateTime::from_timestamp_opt(sess, 0).unwrap().to_string());
}

