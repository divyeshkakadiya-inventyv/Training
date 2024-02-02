use serde_json::{json, Value};
use std::{collections::HashMap, fs, hash};

use crate::services::common_st::{Employee, Position};

///fetch data from the Employee.json and filter out according to the condition and write it in different files after converting into hashmap
pub fn insert_data_using_hashmap() -> String {
    let input_path = "./src/files/Employee.json";

    match fs::read_to_string(input_path) {
        Ok(data) => {
            let json_data: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&data);
            match json_data {
                Ok(json_data) => {
                    let mut filter1 = Vec::new();
                    let mut filter2 = Vec::new();
                    let mut filter3 = Vec::new();

                    for data in json_data {
                        if data.position == Some(Position::Sd)
                            && data.skills.contains(&"Rust".to_string())
                        {
                            filter1.push(data);
                        } else if data.position == Some(Position::Jsd)
                            && data.skills.contains(&"Java".to_string())
                        {
                            filter2.push(data);
                        } else if data.position == Some(Position::Ssd)
                            || data.skills.contains(&"C#".to_string())
                        {
                            filter3.push(data);
                        }
                    }

                    let filter1_map = vec_to_hashmap(filter1);
                    let filter2_map = vec_to_hashmap(filter2);
                    let filter3_map = vec_to_hashmap(filter3);

                    match serde_json::to_string_pretty(&filter1_map) {
                        Ok(data) => {
                            fs::write("./src/files/map_files/filter1_map.json", data);
                        }
                        _ => {}
                    }

                    match serde_json::to_string_pretty(&filter2_map) {
                        Ok(data) => {
                            fs::write("./src/files/map_files/filter2_map.json", data);
                        }
                        _ => {}
                    }

                    match serde_json::to_string_pretty(&filter3_map) {
                        Ok(data) => {
                            fs::write("./src/files/map_files/filter3_map.json", data);
                        }
                        _ => {}
                    }
                    return String::from("Success!");
                }
                Err(err) => {
                    return err.to_string();
                }
            }
        }
        Err(err) => {
            return err.to_string();
        }
    }
}

///convert data of the vector to the hashmap
fn vec_to_hashmap(data: Vec<Employee>) -> Vec<HashMap<String, Value>> {
    let mut final_data: Vec<HashMap<String, Value>> = Vec::new();

    for i in data {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("name".to_string(), Value::String(i.name));
        map.insert("age".to_string(), json!(i.age));
        map.insert("Skills".to_string(), json!(i.skills));
        if !i.position.is_none() {
            map.insert("position".to_string(), json!(i.position));
        }
        if !i.experience.is_none() {
            map.insert("experiance(y)".to_string(), json!(i.experience));
        }
        final_data.push(map);
    }

    final_data
}
