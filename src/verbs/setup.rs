use crate::config::Config;

pub fn exec(username: Option<String>, password: Option<String>, endpoint: Option<String>) -> Result<(), String> {
    if let Ok(config) = Config::new().load_file(){
        eprintln!("Overwriting existing config file.");
        let backup = format!("{}.bak", config.location());
        match std::fs::copy(config.location(), backup)
        {
            Ok(_) => {},
            Err(x) => return Err(format!("Could not backup config file: {}", x).to_string()),
        }
    }
    let cfg = Config::new().load_env().from_options(username, password, endpoint);
    match cfg.save_to_default()
    {
        Ok(_) => {},
        Err(x) => return Err(format!("Could not save config file: {}", x).to_string()),
    } 
    eprintln!("Config file saved to {}", cfg.location());
    Ok(())
}
