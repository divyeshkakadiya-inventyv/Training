use std::cell;

use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::table_task::calculate::calculate_height;

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

///structure for the Inputdata row of json file

#[derive(Debug, Deserialize, Serialize)]
pub struct InputData {
    pub headerRow: HeaderData,
    pub dataRows: RowData,
    pub pageWidth: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderData {
    pub fontSize: usize,
    pub title: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RowData {
    pub fontSize: usize,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell {
    pub cell_content: String,
}

impl Cell {
    pub fn new(cell_content: String) -> Cell {
        Cell { cell_content }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RowType {
    HeaderRow,
    DataRows,
}

#[derive(Debug, Serialize)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub row_height: usize,
    pub row_width: usize,
    pub row_type: RowType,
}

impl Row {
    // pub fn calculate_max_height(cells: &Vec<Cell>) -> usize {
    //     let mut max_height: usize = 0;
    //     for i in 0..cells.len() {
    //         if cells[i].cell_height > max_height {
    //             max_height = cells[i].cell_height;
    //         }
    //     }
    //     max_height
    // }

    pub fn new(
        mut cells: Vec<Cell>,
        row_type: RowType,
        size: usize,
        pagewidth: usize,
        cell_count: &usize,
    ) -> Row {
        println!("{}", cell_count);
        let cell_width = (pagewidth - 10 - 10) / *cell_count;
        let row_width = pagewidth - 10;
        let mut row_height = 0;
        for content in &mut cells {
            let cell_height = calculate_height(&mut content.cell_content, &size, cell_width);
            row_height = std::cmp::max(row_height, cell_height);
            println!("{} : {}", cell_height, row_height);
        }
        println!("{}", row_height);
        Row {
            cells,
            row_height,
            row_width,
            row_type,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub Rows: Vec<Row>,
    pub table_height: usize,
    pub table_width: usize,
}

impl Table {
    pub fn new(Rows: Vec<Row>) -> Table {
        let mut table_height = 0;
        let table_width: usize = Rows.len();
        for i in 0..Rows.len() {
            table_height += Rows[i].row_height
        }
        Table {
            Rows,
            table_height,
            table_width,
        }
    }
}

#[derive(Debug)]
pub struct data {
    pub id: usize,
    pub name: String,
    pub timestamp: DateTime<Utc>,
}

impl data {
    pub fn new(id: usize, name: String, timestamp: DateTime<Utc>) -> data {
        data {
            id,
            name,
            timestamp,
        }
    }
}

//structures for the task_assigner Project
//Employee struct for the task assigner
#[derive(Debug, Deserialize, Serialize)]
pub struct Emp {
    pub id: usize,
    pub name: String,
    pub skills: Vec<String>,
    pub status: Status,
    pub language: Language,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Status {
    Online,
    Offline,
}

impl Status {
    pub fn random() -> Self {
        if rand::thread_rng().gen() {
            Status::Online
        } else {
            Status::Offline
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Language {
    English,
    Spanish,
}

impl Language {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0, 2);
        match random {
            0 => Language::English,
            1 => Language::Spanish,
            _ => todo!("not generated"),
        }
    }
}
//structs use at the runtime for generating random data
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum TaskType {
    Call,
    Chat,
}

impl TaskType {
    pub fn random() -> Self {
        if rand::thread_rng().gen() {
            TaskType::Call
        } else {
            TaskType::Chat
        }
    }
}

#[derive(Debug, Clone , PartialEq)]
pub struct Task {
    pub skill: String,
    pub available_at: TaskType,
    pub language: Language,
    pub time: DateTime<Utc>,
}

impl Task {
    pub fn new(skill: String, available_at: TaskType, language: Language, time: DateTime<Utc>) -> Task {
        Task {
            skill,
            available_at,
            language,
            time,
        }
    }
}
