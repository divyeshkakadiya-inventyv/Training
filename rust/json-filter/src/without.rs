use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    name: String,
    age: u32,
    skills: Vec<String>,
    position: Option<String>,
    // #[serde(rename(serialize = "experiance(y)" , deserialize = "experiance(y)"))]``
    #[serde(rename = "experiance(y)")]
    experiance: Option<u32>,
}

fn main() {
    match fs::read_to_string("./src/Employee.json") {
        Ok(data) => {
            let json_data: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&data);
            match json_data {
                Ok(json_data) => {
                    let mut filter1 = Vec::new();
                    let mut filter2 = Vec::new();
                    let mut filter3 = Vec::new();

                    for data in json_data {
                        if data.position == Some("Software Developer".to_string())
                            && data.skills.contains(&String::from("Rust"))
                        {
                            filter1.push(data);
                        } else if data.position == Some("Jr. Software Developer".to_string())
                            && data.skills.contains(&"Java".to_string())
                        {
                            filter2.push(data);
                        } else if data.position == Some("Sr. Software Developer".to_string())
                            || data.skills.contains(&"c#".to_string())
                        {
                            filter3.push(data);
                        }
                    }

                    match serde_json::to_string_pretty(&filter1) {
                        Ok(data) => {
                            fs::write("./src/filter1.json", data);
                        }
                        _ => {}
                    }

                    match serde_json::to_string_pretty(&filter2) {
                        Ok(data) => {
                            fs::write("./src/filter2.json", data);
                        }
                        _ => {}
                    }

                    match serde_json::to_string_pretty(&filter3) {
                        Ok(data) => {
                            fs::write("./src/filtere.json", data);
                        }
                        _ => {}
                    }
                }
                _ => todo!(),
            }
        }
        Err(_) => todo!(),
    }
}
