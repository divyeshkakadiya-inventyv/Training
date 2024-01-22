use serde::{Deserialize, Serialize};

/// Represents information about a student.
#[derive(Debug, Deserialize, Serialize)]
pub struct Student {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub marks: Vec<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
}

/// Represents information about an employee.
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub age: u32,
    pub skills: Vec<String>,
    pub position: Option<Position>,
    #[serde(rename = "experiance(y)")]
    pub experience: Option<u32>,
}

/// Represents different positions an employee can have.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Position {
    #[serde(rename = "Software Developer")]
    Sd,
    #[serde(rename = "Jr. Software Developer")]
    Jsd,
    #[serde(rename = "Sr. Software Developer")]
    Ssd,
    #[serde(rename = "Project Manager")]
    Pm,
    #[serde(rename = "Team Lead")]
    Tl,
}

/// Represents different skills an employee can have.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Skills {
    Rust(String),
    #[serde(rename = "C#")]
    C(String),
    Java,
    Python,
}
