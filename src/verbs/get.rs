use crate::api;
use crate::config;
use crate::session;
use futures::{stream,StreamExt};
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use indicatif::MultiProgress;
use tokio::io::AsyncWriteExt;
use std::time::Duration;

pub fn exec(ids: Vec<String>, public: Option<bool>) -> Result<(), String>
{
    let public = public.unwrap_or(false);
    let config = config::Config::new().load_all(None,None,None);
    let session = session::Session::new().load_all();
    let multi = MultiProgress::new();
    let flt = api::Flotilla::new(&config, &session);
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap().block_on( download_all(&flt, ids, public, Some(multi)) )
}

#[derive(Debug, Clone)]
struct DownloadTask {
    id: String,
    name: String,
    folder_path: String,
    dl_dest: String,
    meta_url: String,
    dl_url: String,
    client: Client,
    token_value: String,
    postfix: String,
    result: Option<Result<String, String>>,
    bar: Option<ProgressBar>,
}

impl DownloadTask
{
    fn new(id: String, folder_path: String, meta_url: String, dl_url:String, client: Client, token_value: String, postfix: String) -> DownloadTask
    {
        DownloadTask {
            id,
            name: "".to_string(),
            folder_path,
            dl_dest: "".to_string(),
            meta_url,
            dl_url,
            client,
            token_value,
            postfix,
            result: None,
            bar: None,
        }
    }

    async fn get_metadata(&mut self) -> Result<DownloadTask, String>
    {
        let pb = self.bar.as_ref();
        pb.unwrap().inc(1);
        pb.unwrap().set_message(format!("{} [{}] - Getting metadata", self.id, self.postfix));
        let resp = self.client
            .get(&self.meta_url)
            .header("Authorization", &self.token_value)
            .send().await;
        pb.unwrap().set_message(format!("{} [{}] - Parsing metadata", self.id, self.postfix));
        if resp.is_err() {
            return Err(format!("{} - [{}] {} ... Sorry!", self.id, self.postfix, resp.unwrap_err().to_string()));
        }
        let resst = resp.unwrap().error_for_status();
        let bytes = match resst {
            Ok(s) => s.bytes().await,
            Err(e) => {
                return Err(format!("{} [{}] - Denied (are you logged in? Does this exist?) {}", self.id, self.postfix, e.without_url()));
            }
        };
        let v= serde_json::from_slice::<serde_json::Value>(&bytes.unwrap()).unwrap();
        if v["collectionName"].is_null() || v["id"].is_null() {
            return Err(format!("{} [{}] - Bad response from server! ", self.id, self.postfix));
        }
        self.name = v["collectionName"].as_str().unwrap().to_string();
        self.dl_dest = format!("{}/{}-{}-{}.zip", self.folder_path, self.name, self.id[0..8].to_string(), self.postfix);
        Ok(self.clone())
    }

    async fn dl(&mut self) -> Result<DownloadTask, String>
    {
        let pb = self.bar.as_ref();
        pb.unwrap().inc(1);
        pb.unwrap().set_message(format!("{} [{}] - Starting download ... ", self.id, self.postfix));
        let resp = self.client
            .get(&self.dl_url)
            .header("Authorization", &self.token_value)
            .send().await;
        if resp.is_err() {
            return Err(format!("{} - Library error {} ", self.id, resp.unwrap_err().to_string()));
        }
        let resst = resp.unwrap().error_for_status();
        let resp = match resst {
            Ok(x) => x,
            Err(e) => {
                match e.status().unwrap().as_u16() {
                    401 => {
                        return Err(format!("{} [{}] - {} ", self.id, self.postfix, e.without_url()));
                    },
                    404 => {
                        return Err(format!("{} [{}] - Not Found ", self.id, self.postfix ));
                    },
                    500 => {
                        return Err(format!("{} [{}] - Server error - Possibly a bug! {}", self.id, self.postfix,  e.without_url()));
                    },
                    _ => {
                        return Err(format!("{} [{}] - Unknown Error: {} ", self.id, self.postfix, e.without_url()));
                    }
                }
            }
        };
        let mut file = tokio::fs::File::create(&self.dl_dest).await.map_err(|e| e.to_string()).map_err(|e| e.to_string()).unwrap();
        let sz = resp.content_length().unwrap_or(0);
        let mut stream = resp.bytes_stream();
        pb.unwrap().set_length(sz);
        pb.unwrap().set_style(ProgressStyle::default_bar()
                              .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                              .unwrap()
                              .progress_chars("#>-"));
        pb.unwrap().set_message(format!("{} [{}] - Downloading", self.id, self.postfix));
        while let Some(item) = stream.next().await {
            if let Ok(bytes) = item{
                pb.unwrap().inc(bytes.len() as u64);
                match file.write_all(&bytes).await.map_err(|e| e.to_string()) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(format!("{} [{}] - Error writing to file: {}", self.dl_dest, self.postfix, e));
                    }
                }
            } else {
                return Err(format!("{} [{}] - Stream error: {}", self.id, self.postfix, item.unwrap_err().to_string()));
            }
        }
        file.sync_all().await.map_err(|e| e.to_string()).map_err(|e| e.to_string())?;
        Ok( self.clone() )
    }
}

pub async fn download_all<'a>(flt: &api::Flotilla<'a>, ids: Vec<String>, public: bool, multi: Option<MultiProgress>) -> Result<(), String>
{

    let token_value = format!("Bearer {}",flt.session.id_token.replace("\"", ""));
    let client = Client::new();

    let mut tasks = ids.iter().map(|x| {
        match public{
            true =>
                DownloadTask::new(
                    x.to_string(),
                    flt.config.download_path.clone(),
                    format!("{}/shipyard/collection/public/{}", flt.config.endpoint, x),
                    format!("{}/shipyard/collection/download/{}", flt.config.endpoint, x),
                    client.clone(),
                    token_value.clone(),
                    "public ".to_string(),
                    ),
            false =>
                DownloadTask::new(
                    x.to_string(),
                    flt.config.download_path.clone(),
                    format!("{}/shipyard/collection/{}", flt.config.endpoint, x),
                    format!("{}/shipyard/collection/download/{}", flt.config.endpoint, x),
                    client.clone(),
                    token_value.clone(),
                    "private".to_string(),
                    ),
        }
    }).collect::<Vec<DownloadTask>>();

    for task in tasks.iter_mut()
    {
        let pb = multi.clone().unwrap().add(ProgressBar::new(3));
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}"
                )
            .unwrap()
            .progress_chars("#>-"),
            );
        pb.enable_steady_tick( Duration::from_millis(100) );
        pb.set_message(format!("{} [{}] - Starting ... ", task.id, task.postfix));
        task.bar = Some(pb);
    }

    let x = stream::iter(&mut tasks)
        .for_each_concurrent(10, |task|
                             {
                                 let pb = task.bar.clone().unwrap();
                                 async move {
                                     match task.get_metadata().await
                                     {
                                         Ok(_) => {
                                             match task.dl().await
                                             {
                                                 Ok(_) => {
                                                     let ok_msg = format!("{} Downloaded to {}", task.id, task.dl_dest);
                                                     task.result = Some(Ok(ok_msg.clone()));
                                                     pb.finish_with_message(ok_msg);
                                                 },
                                                 Err(e) => {
                                                     task.result = Some(Err(e.clone()));
                                                     pb.abandon_with_message(e.to_string());
                                                 }
                                             }
                                         },
                                         Err(e) => {
                                             task.result = Some(Err(e.clone()));
                                             pb.abandon_with_message(e.to_string());
                                         }
                                     }
                                 }});
    x.await;

    //TODO there's probably a better way to do this
    let errstrings: Vec<String> = tasks
        .iter()
        .filter_map(|x| x.result.as_ref().and_then(|r| r.as_ref().err()))
        .cloned()
        .collect();

    match errstrings.len()
    {
        0 => Ok(()),
        _ => Err(errstrings.join("\n"))
    }

}

