use crate::config;
use crate::session;
use chrono::Utc;

pub fn login(config: &config::Config) -> Result<session::Session, String>
{
    let client = reqwest::blocking::Client::new();

    let json_body = serde_json::json!({
        "email": config.username,
        "password": config.password,
    });

    let mut session = session::load_session();
    eprintln!("Session: {:?}",session);
    if session.expired()
    {
        eprintln!("Session expired, logging in");
        let auth_url = format!("{}/user/quick_login", config.endpoint);
        eprintln!("Auth URL: {}",auth_url);
        let res = match client
            .post(auth_url)
            .header("Content-Type", "application/json")
            .body(json_body.to_string())
            .send()
            {
                Ok(res) => res,
                Err(x) => {
                    eprintln!("Error: Could not connect to server");
                    return Err(x.to_string());
                }
            };

        if res.status().is_success()
        {
            eprintln!("Login successful");
            let json: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
            session.id_token = json["AuthenticationResult"]["IdToken"].to_string();
            session.user_id = config.username.clone();
            session.refresh_token = json["AuthenticationResult"]["RefreshToken"].to_string();
            session.expiration_unix =
                json["AuthenticationResult"]["ExpiresIn"].as_i64().unwrap()+Utc::now().timestamp();
            session::save_session(&session);
        } else {
            eprintln!("Error: Could not log in");
            return Err("Could not log in".to_string());
        }
    }
    Ok(session)
}
