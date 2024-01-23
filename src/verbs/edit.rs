use crate::interface::EditOperation;
use crate::api;
use crate::config::Config;
use crate::session::Session;
use crate::api::UserData;
use serde_json::json;

enum IdType{
    Collection,
    Ship,
}

fn get_id_type(id: &String) -> IdType{
    match id.len()
    {
        32 => IdType::Collection,
        64 => IdType::Ship,
        _ => {
            panic!("Invalid id: {}", id);
        }
    }
}
pub fn exec(id: String, operation: EditOperation){
    dbg!(&id);
    dbg!(&operation);

    let config = Config::new().load_env().load_file().expect("Application Error: Could not load configuration file. Please file a bug!");
    let session = Session::new().load_all();
    if session.expired()
    {
        println!("Session expired. Please login.");
        return;
    }

    let mut user_data = api::Flotilla::new(&config, &session).get_user_data().expect("Application Error: Could not get user data. Please file a bug!");
    println!("{}", serde_json::to_string_pretty(&user_data).unwrap());

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
            } else {
                json_data[&x.key] = json!(x.values[0]);
            }
            json_data
        },
    };

    user_data.collections.iter_mut().for_each(|collection| {
        if collection.id == id
        {
            *collection = serde_json::from_value(new_json_data.clone()).unwrap();
        }
    });

    user_data.ships.iter_mut().for_each(|ship| {
        if ship.id == id
        {
            *ship = serde_json::from_value(new_json_data.clone()).unwrap();
        }
    });

    println!("{}", serde_json::to_string_pretty(&user_data).unwrap());

}

