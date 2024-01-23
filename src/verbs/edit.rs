use crate::interface::EditOperation;
use crate::api::{Flotilla, IdType, get_id_type, Ship, Collection};
use crate::config::Config;
use crate::session::Session;
use similar::{TextDiff, ChangeTag};
use serde_json::json;


pub fn exec(id: String, operation: EditOperation){

    let config = Config::new().load_env().load_file().expect("Application Error: Could not load configuration file. Please file a bug!");
    let session = Session::new().load_all();
    if session.expired()
    {
        eprintln!("Session expired. Please login.");
        return;
    }

    let user_data = Flotilla::new(&config, &session).get_user_data().expect("Application Error: Could not get user data. Please file a bug!");

    let json_data = match get_id_type(&id)
    {
        IdType::Collection => {
            let collection = user_data.collections.iter().find(|collection| collection.id == id)
                .expect(format!("Application Error: Could not find collection with id {}.", id).as_str());
            json!(collection)
        },
        IdType::Ship => {
            let ship = user_data.ships.iter().find(|ship| ship.id == id)
                .expect(format!("Application Error: Could not find ship with id {}.", id).as_str());
            json!(ship)
        },
    };

    let new_json_data = match operation
    {
        EditOperation::Add(x) => {
            let mut json_data = json_data.clone();
            assert!(json_data[&x.key].is_array(), "Cannot add to non-array field");
            x.values
                .iter()
                .for_each(|v| json_data[&x.key].as_array_mut().unwrap()
                          .push(json!(v)));
            json_data
        },
        EditOperation::Remove(x) => {
            let mut json_data = json_data.clone();
            assert!(json_data[&x.key].is_array(), "Cannot remove from non-array field");
            x.values
                .iter()
                .for_each(|v| json_data[&x.key].as_array_mut().unwrap()
                          .retain(|x| x != v));
            json_data
        },
        EditOperation::Set(x) => {
            let mut json_data = json_data.clone();
            if json_data[&x.key].is_array()
            {
                json_data[&x.key] = json!(x.values);
            } else if json_data[&x.key].is_boolean()
            {
                json_data[&x.key] = json!(x.values[0].parse::<bool>().unwrap());
            } else if json_data[&x.key].is_number()
            {
                json_data[&x.key] = json!(x.values[0].parse::<f64>().unwrap());
            } else 
            {
                json_data[&x.key] = json!(x.values[0]);
            }
            json_data
        },
    };

    if json_data == new_json_data
    {
        println!("No changes made.");
        return;
    }

    for change in TextDiff::from_lines(
            &serde_json::to_string_pretty(&json_data).unwrap(), 
            &serde_json::to_string_pretty(&new_json_data).unwrap()
        )
        .iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }

    println!("Are you sure you want to make these changes? (type yes to confirm)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Application Error: Could not read input. Please file a bug!");
    if input.trim().to_ascii_lowercase() != "yes"
    {
        println!("Aborting.");
        return;
    }

    eprintln!("Sending changes to server...");

    match get_id_type(&id)
    {
        IdType::Collection => {
            let collection: Collection = serde_json::from_value(new_json_data).unwrap();
            Flotilla::new(&config, &session).set_collection(collection).expect("Application Error: Could not set collection. Please file a bug!")
        },
        IdType::Ship => {
            let ship: Ship = serde_json::from_value(new_json_data).unwrap();
            Flotilla::new(&config, &session).set_ship(ship).expect("Application Error: Could not set ship. Please file a bug!")
        },
    }

    eprintln!("Changes sent.");

}

