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


///implemantion of the student structure
/// 
impl Student {
    /// Calculates the percentage from the marks of the student.
    pub fn calculate_percentage(&self) -> f32 {
        let sum = self.marks.clone().into_iter().reduce(|a, b| a + b);
        match sum {
            Some(sum) => (sum / self.marks.len() as f32) as f32,
            None => 0.0,
        }
    }

    /// Assigns a grade to the student based on their percentage.
    pub fn grade(&mut self) -> String {
        match self.percentage {
            Some(percentage) => {
                if percentage > 70.0 {
                    "A".to_string()
                } else if percentage > 60.0 && percentage < 70.0 {
                    "B".to_string()
                } else if percentage > 40.0 && percentage < 60.0 {
                    "C".to_string()
                } else {
                    "D".to_string()
                }
            }
            None => "Error is generated in grade Desiding".to_string(),
        }
    }
}