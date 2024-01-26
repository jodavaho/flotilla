use crate::api;
use crate::config;
use crate::session;
use futures::{stream,StreamExt};
use reqwest::Client;
use std::sync::{Arc, Mutex};
//use indicatif::{ProgressBar, ProgressStyle};
use indicatif::MultiProgress;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;

pub fn exec(ids: Vec<String>, public: Option<bool>) -> Result<(), String>
{
    match public{
        Some(true) => {
            Err("Public not implemented yet".to_string())
        }
        Some(false)|None => {
            let config = config::Config::new().load_all(None,None,None);
            let session = session::Session::new().load_all();
            let _multi = Arc::new(Mutex::new(MultiProgress::new()));
            let flt = api::Flotilla::new(&config, &session);
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap().block_on( download_all(&flt, ids) )
        }

    }
}

struct DownloadTask {
    id: String,
    name: String,
    folder_path: String,
    dl_dest: String,
    meta_url: String,
    dl_url: String,
    size: u64,
    client: Client,
    token_value: String,
    data: Option<serde_json::Value>,
}

impl Clone for DownloadTask {
    fn clone(&self) -> Self {
        DownloadTask {
            id: self.id.clone(),
            name: self.name.clone(),
            folder_path: self.folder_path.clone(),
            dl_dest: self.dl_dest.clone(),
            meta_url: self.meta_url.clone(),
            dl_url: self.dl_url.clone(),
            size: self.size.clone(),
            client: self.client.clone(),
            token_value: self.token_value.clone(),
            data: self.data.clone(),
        }
    }
}
impl DownloadTask
{
    fn new(id: String, folder_path: String, meta_url: String, dl_url:String, client: Client, token_value: String) -> DownloadTask
    {
        DownloadTask {
            id,
            name: "".to_string(),
            folder_path,
            dl_dest: "".to_string(),
            meta_url,
            dl_url,
            size: 0,
            client,
            token_value,
            data: None,
        }
    }

    async fn get_metadata(&mut self) -> Result<DownloadTask, String>
    {
        let resp = self.client
            .get(&self.meta_url)
            .header("Authorization", &self.token_value)
            .send().await;
        if resp.is_err() {
            return Err(format!("Library error fetching metadata for {} from {} ... Sorry!", self.id, self.meta_url));
        }
        let resst = resp.unwrap().error_for_status();
        let bytes = match resst {
            Ok(s) => s.bytes().await,
            Err(e) => {
                return Err(format!("Error fetching metadata for {} - is it public or are you logged in? {}", self.id, e.without_url()));
            }
        };
        let v= serde_json::from_slice::<serde_json::Value>(&bytes.unwrap()).unwrap();
        if v["collectionName"].is_null() || v["id"].is_null() {
            return Err(format!("Error fetching metadata, bad server response for {}", self.id));
        }
        self.name = v["collectionName"].as_str().unwrap().to_string();
        self.dl_dest = format!("{}/{}-{}.zip", self.folder_path, self.name, self.id[0..8].to_string());
        Ok(self.clone())
    }

    async fn dl(&mut self) -> Result<DownloadTask, String>
    {
        let resp = self.client
            .get(&self.dl_url)
            .header("Authorization", &self.token_value)
            .send().await;
        if resp.is_err() {
            return Err(format!("Library error while downloading {} aka {} ... Sorry! ", self.id, self.name));
        }
        let resst = resp.unwrap().error_for_status();
        let resp = match resst {
            Ok(x) => x,
            Err(e) => {
                match e.status().unwrap().as_u16() {
                    401 => {
                        return Err(format!("Implementation erorr while downloading {} aka {} - you might have an old version? {}", self.id, self.name, e.without_url()));
                    },
                    404 => {
                        return Err(format!("Error downloading {} aka {} - is it public or are you logged in? {}", self.id, self.name, e.without_url()));
                    },
                    500 => {
                        return Err(format!("Server error while downloading {} aka {} - Possibly a bug! {}", self.id, self.name, e.without_url()));
                    },
                    _ => {
                        return Err(format!("Other error while downloading {} aka {} - {}", self.id, self.name, e.without_url()));
                    }
                }
            }
        };
        self.size = resp.content_length().unwrap();
        let mut file = tokio::fs::File::create(&self.dl_dest).await.map_err(|e| e.to_string()).map_err(|e| e.to_string()).unwrap();
        let mut stream = resp.bytes_stream();
        while let Some(item) = stream.next().await {
            let bytes = item.unwrap();
            match file.write_all(&bytes).await.map_err(|e| e.to_string()) {
                Ok(_) => {},
                Err(e) => {
                    return Err(format!("Error writing to file {} - {}", self.dl_dest, e));
                }
            }
        }
        file.sync_all().await.map_err(|e| e.to_string()).map_err(|e| e.to_string())?;
        Ok( self.clone() )
    }
}
pub async fn download_all<'a>(flt: &api::Flotilla<'a>, ids: Vec<String>) -> Result<(), String>
{

    let token_value = format!("Bearer {}",flt.session.id_token.replace("\"", ""));
    let client = Client::new();

    let tasks = ids.iter().flat_map(|x| {
        vec![
        DownloadTask::new(
            x.to_string(),
            flt.config.download_path.clone(),
            format!("{}/shipyard/collection/public/{}", flt.config.endpoint, x),
            format!("{}/shipyard/collection/download/{}", flt.config.endpoint, x),
            client.clone(),
            token_value.clone(),
        ),
        DownloadTask::new(
            x.to_string(),
            flt.config.download_path.clone(),
            format!("{}/shipyard/collection/{}", flt.config.endpoint, x),
            format!("{}/shipyard/collection/download/{}", flt.config.endpoint, x),
            client.clone(),
            token_value.clone(),
        ), ]
    }).collect::<Vec<DownloadTask>>();

    let collecs = HashMap::<String, Result<(),String>>::new();

    let collmut = Arc::new(Mutex::new(collecs));
    let tasks_and_coll = tasks.into_iter().map(|x| (x, collmut.clone())).collect::<Vec<(DownloadTask, Arc<Mutex<HashMap<String, Result<(),String>>>>)>>();

    //Get metadata for all collections
    stream::iter(tasks_and_coll)
        .for_each_concurrent(10, 
                             |(mut task, c)| 
                             async move {
                                 let id = task.id.clone();
                                 match task.get_metadata().await
                                 {
                                     Ok(_) => {
                                         match task.dl().await
                                         {
                                             Ok(t) => {
                                                 println!("{} - ( {} ) Downloaded to {}", t.id, t.name, t.dl_dest);
                                                 c.lock().unwrap().insert(id.clone(), Ok(()));
                                             },
                                             Err(e) => {
                                                 c.lock().unwrap().insert(id.clone(), Err(e.to_string()));
                                                 return;
                                             }
                                         }
                                     },
                                     Err(e) => {
                                         c.lock().unwrap().insert(id.clone(), Err(e.to_string()));
                                         return;
                                     }
                                 }
                             }).await;

    let coll = collmut.lock().unwrap();
    let mut errs = vec![];
    for (k,v) in coll.iter() {
        match v {
            Ok(_) => {},
            Err(e) => {
                errs.push(format!("{} - {}", k, e));
            }
        }
    }
    if errs.len() > 0 {
        return Err(errs.join("\n"));
    }

    Ok(())

}

