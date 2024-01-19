use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    name: String,
    age: u32,
    skills: Vec<String>,
    position: Option<Position>,
    // #[serde(rename(serialize = "experiance(y)" , deserialize = "experiance(y)"))]``
    #[serde(rename = "experiance(y)")]
    experiance: Option<u32>,
}

#[derive(Deserialize , Serialize , Debug , PartialEq)]
enum Position{
    #[serde(rename = "Software Developer")]
    Sd,
    #[serde(rename = "Jr. Software Developer")]
    Jsd,
    #[serde(rename = "Sr. Software Developer")]
    Ssd,
    #[serde(rename = "Project Manager")]
    Pm,
    #[serde(rename = "Team Lead")]
    Tl
}

#[derive(Deserialize , Serialize , Debug , PartialEq)]
enum Skills{
    Rust(String),
    #[serde(rename = "C#")]
    C(String),
    Java,
    Python
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
                Err(err) => {
                    println!("{:?}" , err);
                },
            }
        }
        Err(_) => todo!(),
    }
}
