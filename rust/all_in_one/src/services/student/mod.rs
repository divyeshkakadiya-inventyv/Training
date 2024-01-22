use crate::services::common_st::Student;
use std::fs;

impl Student {
    /// Calculates the percentage from the marks of the student.
    fn calculate_percentage(&self) -> f32 {
        let sum = self.marks.clone().into_iter().reduce(|a, b| a + b);
        match sum {
            Some(sum) => (sum / self.marks.len() as f32) as f32,
            None => 0.0,
        }
    }

    /// Assigns a grade to the student based on their percentage.
    fn grade(&mut self) -> String {
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

/// Inserts data from the "StudentData.json" file into a new JSON file with added percentages and grades.
pub fn insert_data() -> &'static str {
    // Read the contents in a string
    let json_string =
        fs::read_to_string("./src/files/StudentData.json").expect("Couldn't Perform Operation");

    println!("{:?}", json_string);

    //convert data into the vector of structure
    let student_data: Vec<Student> =
        serde_json::from_str(&json_string).expect("Failed to deserialize.");

    println!("{:#?}", student_data); //# is use to display pretty JSON format

    // add percentage into all structs
    let updated_data: Vec<Student> = student_data
        .into_iter()
        .map(|mut data| {
            data.percentage = Some(data.calculate_percentage());
            data.grade = Some(data.grade());
            data
        })
        .collect();

    println!("{:#?}", updated_data);

    let final_data =
        serde_json::to_string_pretty(&updated_data).expect("Data is not converted to JSON");
    fs::write("./src/files/student.json", final_data).expect("Data is not written to file");

    "Success"
}
