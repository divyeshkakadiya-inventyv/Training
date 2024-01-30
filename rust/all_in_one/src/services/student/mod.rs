use crate::services::common_st::Student;

use std::fs;

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
