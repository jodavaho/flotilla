use crate::config;
use crate::session::Session;

pub fn exec() -> Result<(), String>
{
    if let Err(x) = config::Config::new().load_file().expect("Could not load config file").remove()
    {
        return Err(format!("Could not remove config file: {}", x));
    }
    if let Err(x) = Session::new().remove()
    {
        return Err(format!("Could not remove session file: {}", x));
    }
    eprintln!("Logged out.");
    Ok(())
}
