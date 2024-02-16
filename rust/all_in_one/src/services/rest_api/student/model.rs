use serde::{Deserialize, Serialize};


#[derive(Debug , Serialize, Deserialize , Clone , PartialEq)]
pub struct Student {
    pub id : i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub marks: Vec<f32>,
}