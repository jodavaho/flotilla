use crate::config::Config;
use crate::api;
pub fn exec(username: Option<String>, password: Option<String>, endpoint: Option<String>)
{

    let config = Config::new().load_all(username, password, endpoint);
    api::login(&config)
        .expect("Could not log in")
        .save_to_default();
}

