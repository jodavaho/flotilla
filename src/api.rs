use crate::config;
use crate::session;
use chrono::Utc;

pub fn login(config: &config::Config) -> Result<session::Session, String>
{

    eprintln!("Logging in to {}", config.endpoint);
    let client = reqwest::blocking::Client::new();

    let json_body = serde_json::json!({
        "email": config.username,
        "password": config.password,
    });

    let mut session = session::load_session();
    if session.expired()
    {
        eprintln!("Session expired, logging in");
        let auth_url = format!("{}/user/quick_login", config.endpoint);
        eprintln!("Auth URL: {}",auth_url);
        let res = client
            .post(auth_url)
            .header("Content-Type", "application/json")
            .body(json_body.to_string())
            .send();

        eprintln!("Response: {:?}",res);

        if res.is_err()
        {
            eprintln!("Error: Could not connect to server");
            return Err(res.err().unwrap().to_string());
        };
        let res = res.unwrap();

        eprintln!("Login successful");
        let json: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
        session.id_token = json["AuthenticationResult"]["IdToken"].to_string();
        session.user_id = config.username.clone();
        session.refresh_token = json["AuthenticationResult"]["RefreshToken"].to_string();
        session.expiration_unix =
            json["AuthenticationResult"]["ExpiresIn"].as_i64().unwrap()+Utc::now().timestamp();
        session::save_session(&session);
    }
    Ok(session)
}
