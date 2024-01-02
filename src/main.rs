
mod interface;
mod config;
mod session;
mod api;

fn main() {
    let matches = interface::parse_cli();

    if matches.get_flag("setup")
    {
        let config = config::load_all();
        config::save_file(config);
        return;
    }


    let config = config::load_all();
    eprintln!("Hello, {}", config.username);
    eprintln!("Loading flotilla from {}", config.endpoint);
    eprintln!("config: API endpoint:{}",config.endpoint);

    let session = api::login(&config).expect("Could not log in");

    let app = api::Flotilla { config, session };
    let user_data = app.get_user_data().expect("Could not get user data");
    eprintln!("User data: {:?}", user_data);
}
