use crate::api;
use crate::config;
use crate::session;

pub fn exec(ids: Vec<String>, public: Option<bool>) -> Result<(), String>
{
    match public{
        Some(true) => {
            Err("Public not implemented yet".to_string())
        }
        Some(false)|None => {
            let config = config::Config::new().load_all(None,None,None);
            let session = session::Session::new().load_all();
            let _ = api::Flotilla::new(&config, &session).download_collections(ids);
            eprintln!("Done");
            Ok(())
        }
    }
}
