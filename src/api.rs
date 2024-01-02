use crate::config;
use crate::session;
use chrono::Utc;

#[derive(Debug)]
pub struct Flotilla<'a> {
    pub config: &'a config::Config,
    pub session: &'a session::Session,
}

#[derive(Debug)]
pub struct Ship {
    pub id: String,
    pub name: String,
    pub file_name: String,
    pub download_url: String,
    pub description: String,
    pub owner: String,
    pub created: String,
    pub public_url: String,
    pub short_id: String,
    pub downloads: u32,
    pub uploaded_unix: i64,
}

#[derive(Debug)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub public_url: String,
    pub ship_ids: Vec<String>,
}

pub fn login(config: &config::Config) -> Result<session::Session, String>
{

    eprintln!("Logging in to {}", config.endpoint);
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
        dbg!("Auth URL: {}", &auth_url);

        let res = client
            .post(auth_url)
            .header("Content-Type", "application/json")
            .body(json_body.to_string())
            .send();
        dbg!("Response: {:?}", &res);

        if res.is_err()
        {
            return Err(res.err().unwrap().to_string());
        };
        let res = res.unwrap();

        let json: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
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
            config: config,
            session: session,
        }
    }

    #[allow(dead_code)]
    pub fn get_user_data(&self) -> Result<(Vec<Ship>, Vec<Collection>), String>
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
            eprintln!("Error: Could not connect to server");
            return Err(res.err().unwrap().to_string());
        };
        let res = res.unwrap();

        let json: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
        eprintln!("User data: {}", json);
        let ships = json["ships"].as_array().unwrap();
        let mut ship_list: Vec<Ship> = Vec::new();
        for ship in ships
        {
            let ship_id = ship["id"].to_string();
            let ship_name = ship["shipName"].to_string();
            let ship_description = ship["description"].to_string();
            let ship_owner = ship["owner"].to_string();
            let ship_created = ship["created"].to_string();
            let ship_file_name = ship["fileName"].to_string();
            let ship_download_url = ship["downloadUrl"].to_string();
            let ship_public_url = ship["publicUrl"].to_string();
            let ship_downloads = ship["downloads"].as_u64().unwrap();
            let ship_short_id = ship["shortId"].to_string();
            let ship_uploaded_unix = ship["uploaded"].as_i64().unwrap();

            let ship = Ship {
                id: ship_id,
                name: ship_name,
                description: ship_description,
                owner: ship_owner,
                file_name: ship_file_name,
                download_url: ship_download_url,
                public_url: ship_public_url,
                created: ship_created,
                downloads: ship_downloads as u32,
                short_id: ship_short_id,
                uploaded_unix: ship_uploaded_unix,
            };
            ship_list.push(ship);
        }
        let collections = json["collections"].as_array().unwrap();
        let mut collection_list: Vec<Collection> = Vec::new();
        for collection in collections
        {
            let collection_id = collection["id"].to_string();
            let collection_name = collection["collectionName"].to_string();
            let collection_description = collection["description"].to_string();
            let collection_ship_ids = collection["ships"].as_array().unwrap();
            let collection_public_url = collection["publicUrl"].to_string();
            let mut ship_ids: Vec<String> = Vec::new();
            for ship_id in collection_ship_ids
            {
                let ship_id = ship_id.to_string();
                ship_ids.push(ship_id);
            }
            let collection = Collection {
                id: collection_id,
                name: collection_name,
                description: collection_description,
                public_url: collection_public_url,
                ship_ids: ship_ids,
            };
            collection_list.push(collection);
        }
        Ok((ship_list,collection_list))
    }
}
