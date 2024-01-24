use crate::config;
use crate::session;
use chrono::Utc;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::cmp::min;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use futures_util::StreamExt;

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

trait HasId
{
    fn get_id(&self) -> String;
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
impl HasId for Ship
{
    fn get_id(&self) -> String
    {
        self.id.clone()
    }
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
    #[serde(rename = "collectionOwner")]
    pub owner: String,
}
impl HasId for Collection
{
    fn get_id(&self) -> String
    {
        self.id.clone()
    }
}


pub enum IdType{
    Collection,
    Ship,
}

pub fn get_id_type(id: &String) -> IdType{
    match id.len()
    {
        32 => IdType::Collection,
        64 => IdType::Ship,
        _ => {
            panic!("Invalid id: {}", id);
        }
    }
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
   


    pub fn get_json_by_id(&self, id: &String)  -> Result<serde_json::Value, String>{
        match get_id_type(id)
        {
            IdType::Collection => {
                self.get_collection(id).map_err(|e| e.to_string()).map(|c| json!(c))
            },
            IdType::Ship => {
                self.get_ship(id).map_err(|e| e.to_string()).map(|s| json!(s))
            }
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
    pub fn set_by_id(&self, id: &String, json: serde_json::Value) -> Result<(), String>
    {
        return match get_id_type(id)
        {
            IdType::Collection => self.set_collection( serde_json::from_value(json).unwrap()),
            IdType::Ship => self.set_ship( serde_json::from_value(json).unwrap()),
        }
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

    pub fn get_collection(&self, id: &String) -> Result<Collection, String>
    {
        let token_value = format!("Bearer {}",self.session.id_token.replace("\"", ""));
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/shipyard/collection/{}", self.config.endpoint, id);
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
        let data: Collection = serde_json::from_str(&txt).unwrap();

        Ok(data)
    }

    pub fn get_ship(&self, id: &String) -> Result<Ship, String>
    {
        let token_value = format!("Bearer {}",self.session.id_token.replace("\"", ""));
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/shipyard/ship/{}", self.config.endpoint, id);
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
        let data: Ship = serde_json::from_str(&txt).unwrap();

        Ok(data)
    }

    pub fn get_public_collection(&self, id: &String) -> Result<Collection, String>
    {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/shipyard/collection/public/{}", self.config.endpoint, id);
        let res = client
            .get(url)
            .send();

        if res.is_err()
        {
            return Err(res.err().unwrap().to_string());
        };
        let res = res.unwrap();
        let txt = res.text().unwrap();
        let data: Collection = serde_json::from_str(&txt).unwrap();

        Ok(data)
    }

    /* Download the .zip file of a collection */
    pub fn download_collections(&self, ids: Vec<String>) -> Result<(), String>
    {
        let multi = MultiProgress::new();
        let mut handles = vec![];
        for id in ids
        {
            let meta = match self.get_public_collection(&id)
            {
                Ok(meta) => meta,
                Err(_) => {
                    eprintln!("Collection {} not found or could not fetch metadata... skipping", id);
                    continue;
                }
            };
            
            let path = format!("{}/{}.zip", self.config.download_path, meta.name);
            let url = format!("{}/shipyard/collection/download/{}", self.config.endpoint, id);
            // Indicatif setup
            let pb = ProgressBar::new(100);
            let pb = multi.add(pb);
            // TODO since we already use tokio, just make this a tokio spawn.
            let handle = std::thread::spawn(move|| {
                let worker_future = download_worker(&url, &path, &pb);
                let res = futures::executor::block_on( worker_future );
                if res.is_err()
                {
                    eprintln!("Failed to download collection {} : {}", id, res.err().unwrap());
                }
                else
                {
                    eprintln!("Downloaded collection {}", id);
                }
            });
            handles.push(handle);
        }
        for handle in handles
        {
            handle.join().unwrap();
        }
        multi.clear().unwrap();
        Ok(())
    }

}

#[allow(dead_code)]
pub async fn download_worker(url:&String, path:&String, pb: &ProgressBar ) -> Result<(),String>
    {

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .send()
            .await;
        //.await;

        if res.is_err()
        {
            return Err(res.err().unwrap().to_string());
        };
        let res = res.unwrap();

        let total_size = res
            .content_length()
            .ok_or(format!("Failed to get content length from '{}'", &url))?;

        pb.set_style(
            ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", url.clone()));

        // download chunks
        let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await{
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write_all(&chunk)
                .or(Err(format!("Error while writing to file")))?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            let pct : f64 = new as f64 / total_size as f64;
            pb.set_position( (pct*100.0) as u64);
        }

        pb.finish_with_message(format!("Downloaded {} to {}", url, path));
        Ok(())
    }
