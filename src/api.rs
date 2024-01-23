use crate::config;
use crate::session;
use chrono::Utc;
use serde_json::json;

#[derive(Debug)]
pub struct Flotilla<'a> {
    pub config: &'a config::Config,
    pub session: &'a session::Session,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserData
{
    pub ships: Vec<Ship>,
    pub collections: Vec<Collection>,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ship {
    pub id: String,
    #[serde(rename = "shipName")]
    pub name: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "shortId")]
    pub short_id: String,
    pub downloads: u32,
    pub uploaded: u64,
    #[serde(rename = "numCollections")]
    pub num_collections: u32,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Collection {
    pub id: String,
    #[serde(rename = "collectionName")]
    pub name: String,
    pub description: String,
    #[serde(rename = "publicUrl")]
    pub public_url: String,
    #[serde(rename = "ships")]
    pub ship_ids: Vec<String>,
    pub icon: String,
    pub color: String,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
}

pub fn login(config: &config::Config) -> Result<session::Session, String>
{

    let client = reqwest::blocking::Client::new();

    let json_body = serde_json::json!({
        "email": config.username,
        "password": config.password,
    });


    let mut session = session::Session::new();
    session.load_all();
    if session.expired()
    {
        eprintln!("Session expired, logging in");
        let auth_url = format!("{}/user/quick_login", config.endpoint);

        let res = client
            .post(auth_url)
            .header("Content-Type", "application/json")
            .body(json_body.to_string())
            .send();

        if res.is_err()
        {
            return Err(res.err().unwrap().to_string());
        };
        let res = res.unwrap();
        let txt = res.text().unwrap();

        let json: serde_json::Value = serde_json::from_str(&txt).unwrap();
        session.id_token = json["AuthenticationResult"]["IdToken"].to_string();
        session.user_id = config.username.clone();
        session.refresh_token = json["AuthenticationResult"]["RefreshToken"].to_string();
        session.expiration_unix =
            json["AuthenticationResult"]["ExpiresIn"].as_i64().unwrap()+Utc::now().timestamp();
        session.save_to_default();
    }
    Ok(session)
}

impl<'a> Flotilla<'a>{
    #[allow(dead_code)]
    pub fn new(config: &'a config::Config, session: &'a session::Session) -> Self {
        Self {
            config,
            session
        }
    }

    #[allow(dead_code)]
    pub fn get_user_data(&self) -> Result<UserData, String>
    {

        let token_value = format!("Bearer {}",self.session.id_token.replace("\"", ""));
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/user", self.config.endpoint);
        let res = client
            .get(url)
            .header("Authorization", token_value)
            .send();

        if res.is_err()
        {
            return Err(res.err().unwrap().to_string());
        };
        let res = res.unwrap();
        let txt = res.text().unwrap();
        let data: UserData = serde_json::from_str(&txt).unwrap();

        Ok(data)
    }

    #[allow(dead_code)]
    pub fn set_collection(&self, collection: Collection) -> Result<(), String>
    {
        let id = collection.id.clone();
        let collection = json!(collection);
        let token_value = format!("Bearer {}",self.session.id_token.replace("\"", ""));
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/shipyard/collection/{}", self.config.endpoint, id);
        let res = client
            .put(url)
            .header("Authorization", token_value)
            .body(collection.to_string())
            .send();

        if res.is_err()
        {
            return Err(res.err().unwrap().to_string());
        }
        let res = res.unwrap();
        let _txt = res.text().unwrap();
        Ok(())

    }

    #[allow(dead_code)]
    pub fn set_ship(&self, ship: Ship) -> Result<(), String>
    {
        let id = ship.id.clone();
        let ship = json!(ship);
        let token_value = format!("Bearer {}",self.session.id_token.replace("\"", ""));
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/shipyard/ship/{}", self.config.endpoint, id);
        let res = client
            .put(url)
            .header("Authorization", token_value)
            .body(ship.to_string())
            .send();

        if res.is_err()
        {
            return Err(res.err().unwrap().to_string());
        }
        let res = res.unwrap();
        let _txt = res.text().unwrap();
        Ok(())

    }

}
