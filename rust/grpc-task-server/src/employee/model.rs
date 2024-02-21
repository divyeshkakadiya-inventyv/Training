use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize , Clone)]
pub struct Empl{
    pub id : i32,
    pub name: String,
    pub age: u32,
    pub skills: Vec<String>,
    pub position: Option<String>,
    #[serde(rename = "experiance(y)")]
    pub experience: Option<u32>,
}