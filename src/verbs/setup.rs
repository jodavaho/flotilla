
pub fn exec(username: Option<String>, password: Option<String>, endpoint: Option<String>) {
    if let Ok(config) = Config::new().load_file(){
        eprintln!("Overwriting existing config file.");
        let backup = format!("{}.bak", config.location());
        if let Err(x) = std::fs::copy(config.location(), backup)
        {
            eprintln!("Could not back up config file: {}", x);
        }
    }
    let cfg = Config::new().load_env().from_options(username, password, endpoint);
    if let Err(x) = cfg.save_to_default()
    {
        eprintln!("Could not save config file: {}", x);
    } 
    eprintln!("Config file saved to {}", cfg.location());
}
