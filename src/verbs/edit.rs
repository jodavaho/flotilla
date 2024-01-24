use crate::interface::EditOperation;
use crate::api::{Flotilla, IdType, get_id_type, Ship, Collection};
use crate::config::Config;
use crate::session::Session;
use similar::{TextDiff, ChangeTag};
use serde_json::json;


pub fn exec(id: String, operation: EditOperation) -> Result<(), String>{

    let config = Config::new().load_env().load_file().map_err(|e| format!("Application Error: Could not load configuration file. Please file a bug! {}", e))?;
    let session = Session::new().load_all();
    if session.expired()
    {
        Err("Session expired. Please login.".to_string())?;
    }

    let json_data = Flotilla::new(&config, &session).get_json_by_id(&id)?;

    if json_data.is_null()
    {
        return Err("Could not find object with that ID".to_string());
    }
    let mut new_json_data = json_data.clone();
    new_json_data = match operation
    {
        EditOperation::Add(x) => {
            assert!(new_json_data[&x.key].is_array(), "Cannot add to non-array field");
            x.values
                .iter()
                .for_each(|v| new_json_data[&x.key].as_array_mut().unwrap()
                          .push(json!(v)));
            new_json_data
        },
        EditOperation::Remove(x) => {
            if !new_json_data[&x.key].is_array()
            {
                return Err("Cannot remove from non-array field".to_string());
            }
            assert!(new_json_data[&x.key].is_array(), "Cannot remove from non-array field");
            x.values
                .iter()
                .for_each(|v| new_json_data[&x.key].as_array_mut().unwrap()
                          .retain(|x| x != v));
            new_json_data
        },
        EditOperation::Set(x) => {

            if new_json_data[&x.key].is_array()
            {
                new_json_data[&x.key] = json!(x.values);
            } else if new_json_data[&x.key].is_boolean()
            {
                new_json_data[&x.key] = json!(x.values[0].parse::<bool>().unwrap());
            } else if new_json_data[&x.key].is_number()
            {
                new_json_data[&x.key] = json!(x.values[0].parse::<f64>().unwrap());
            } else 
            {
                new_json_data[&x.key] = json!(x.values[0]);
            }
            new_json_data
        },
    };

    if json_data == new_json_data
    {
        println!("No changes made.");
        return Ok(());
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
    std::io::stdin().read_line(&mut input).map_err(|e| format!("Application Error: Could not read input. Please file a bug! {}", e))?;
    if input.trim().to_ascii_lowercase() != "yes"
    {
        println!("Aborting.");
        return Ok(());
    }

    eprintln!("Sending changes to server...");

    match get_id_type(&id)
    {
        IdType::Collection => {
            let collection: Collection = serde_json::from_value(new_json_data).unwrap();
            Flotilla::new(&config, &session).set_collection(collection).map_err(|e| format!("Application Error: Could not set collection. Please file a bug! {}", e))?;
        },
        IdType::Ship => {
            let ship: Ship = serde_json::from_value(new_json_data).unwrap();
            Flotilla::new(&config, &session).set_ship(ship).map_err(|e| format!("Application Error: Could not set ship. Please file a bug! {}", e))?;
        },
    }

    eprintln!("Changes sent.");
    Ok(())

}

