use super::common_st::{Employee, Skills, Position};
use serde::{Deserialize, Serialize};
use std::fs;

/// Inserts data into different files based on filtering condition.
///
/// Reads data from the "./src/files/Employee.json" file, filters it based on
/// specific conditions and writes the data to separate JSON files.
pub fn insert_data() -> String {
    match fs::read_to_string("./src/files/Employee.json") {
        Ok(data) => {
            let json_data: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&data);
            match json_data {
                Ok(json_data) => {
                    let mut filter1 = Vec::new();
                    let mut filter2 = Vec::new();
                    let mut filter3 = Vec::new();

                    for data in json_data {
                        if data.position == Some(Position::Sd) && data.skills.contains(&"Rust".to_string()) {
                            filter1.push(data);
                        } else if data.position == Some(Position::Jsd) && data.skills.contains(&"Java".to_string()) {
                            filter2.push(data);
                        } else if data.position == Some(Position::Ssd) || data.skills.contains(&"C#".to_string()) {
                            filter3.push(data);
                        }
                    }

                    match serde_json::to_string_pretty(&filter1) {
                        Ok(data) => {
                            fs::write("./src/files/filter1.json", data);
                        }
                        _ => {}
                    }

                    match serde_json::to_string_pretty(&filter2) {
                        Ok(data) => {
                            fs::write("./src/files/filter2.json", data);
                        }
                        _ => {}
                    }

                    match serde_json::to_string_pretty(&filter3) {
                        Ok(data) => {
                            fs::write("./src/files/filter3.json", data);
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
        Err(_) => todo!(),
    }
}
