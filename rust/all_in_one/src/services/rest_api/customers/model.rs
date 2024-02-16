use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub skills: Vec<String>,
    pub status: String,
    pub language: String,
}