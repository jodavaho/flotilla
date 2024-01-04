
use crate::config;
use crate::session::Session;

pub fn exec()
{
    match config::Config::new().load_file().expect("Could not load config file").remove()
    {
        Ok(_) => eprintln!("Config removed"),
        Err(x) => eprintln!("Could not remove config: {}", x),
    }
    match Session::new().remove()
    {
        Ok(_) => eprintln!("Sesison removed"),
        Err(x) => eprintln!("Could not remove session: {}", x),
    }
}
