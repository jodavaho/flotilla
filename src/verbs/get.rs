use crate::api;
use crate::config;
use crate::session;
use futures::{stream,StreamExt};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
//use indicatif::{ProgressBar, ProgressStyle};
use indicatif::MultiProgress;
//use std::fs::File;

pub fn exec(ids: Vec<String>, public: Option<bool>) -> Result<(), String>
{
    match public{
        Some(true) => {
            Err("Public not implemented yet".to_string())
        }
        Some(false)|None => {
            let config = config::Config::new().load_all(None,None,None);
            let session = session::Session::new().load_all();
            let multi = Arc::new(Mutex::new(MultiProgress::new()));
            let flt = api::Flotilla::new(&config, &session);
            let _ = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap().block_on( download_all(&flt, ids, multi.clone()) ).unwrap();
            Ok(())
        }

    }
}

pub async fn download_all<'a>(flt: &api::Flotilla<'a>, ids: Vec<String>, _multi: Arc<Mutex<MultiProgress>>) -> Result<(), String>
{

    let token_value = format!("Bearer {}",flt.session.id_token.replace("\"", ""));
    let client = Client::new();
    let collections = HashMap::<String,api::Collection>::new();

    let public_urls = ids.iter().map(|x| {
        format!("{}/shipyard/collection/public/{}", flt.config.endpoint, x)
    }).collect::<Vec<String>>();

    let private_urls = ids.iter().map(|x| {
        format!("{}/shipyard/collection/{}", flt.config.endpoint, x)
    }).collect::<Vec<String>>();

    let maybe_text = stream::iter(public_urls).chain(stream::iter(private_urls))
        .map(|url|
             {
                 let client = &client;
                 let token_value = &token_value;
                 async move {
                     //eprintln!("Fetching {}", url);
                     let resp = client
                         .get(&url)
                         .header("Authorization", token_value.clone())
                         .send().await;
                     resp?.bytes().await
                 }
             })
    .buffer_unordered(10);

    let coll_lock = Arc::new(Mutex::new(collections));

    eprintln!("Downloading {} collections", ids.len());

    maybe_text.for_each_concurrent(10, |b| async {
        match b
        {
            Ok(bytes) => {
                //eprintln!("Got {} bytes as {}", bytes.len(), String::from_utf8_lossy(&bytes));
                if let Ok(collection) = serde_json::from_slice::<api::Collection>(&bytes)
                {
                    let mut coll = coll_lock.lock().unwrap();
                    coll.insert(collection.id.clone(), collection);
                }
            },
            Err(x) => {
                eprintln!("Error: {}", x);
            }
        }
    }).await;

    for id in ids
    {
        match coll_lock.lock().unwrap().get(&id)
        {
            Some(c) => {
                let path = format!("{}/{}-{}.zip", flt.config.download_path, c.name,c.id[0..8].to_string());
                let url = format!("{}/shipyard/collection/download/{}", flt.config.endpoint, id);
                eprintln!("Downloading {} to {}", url, path);
                //let pb = ProgressBar::new(100);
                //let pb = multi.lock().unwrap().add(pb);
                /*
                match download_worker(&url, &path, &pb, &client).await
                {
                    Ok(_) => {
                        pb.finish_with_message(format!("Downloaded {}", c.name));
                    },
                    Err(e) => {
                        pb.finish_with_message(format!("Error: {}", e));
                    }
                }*/
            },
            None => {
                eprintln!("Collection {} not found or could not fetch metadata... skipping", id);
            }
        }
    }

    Ok(())
}

