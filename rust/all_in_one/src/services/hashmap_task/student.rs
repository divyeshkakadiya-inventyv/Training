use serde_json::{json, Value};

use crate::services::common_st::Student;
use std::{collections::HashMap, fs};

/// Inserts data from the "StudentData.json" file into a new JSON file with added percentages and grades after converting the data to the hashmap.
pub fn insert_data_using_hashmap() -> String {
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

    // println!("{:#?}", updated_data);

    let updated_data = vec_to_hashmap(updated_data);

    let final_data =
        serde_json::to_string_pretty(&updated_data).expect("Data is not converted to JSON");

    match fs::write("./src/files/map_files/student_map.json", final_data) {
        Ok(_) => return "Success".to_string(),
        Err(err) => return err.to_string(),
    }
}

///converts all the vector to hashmap of the perticular data
fn vec_to_hashmap(data: Vec<Student>) -> Vec<HashMap<String, Value>> {
    let mut final_data: Vec<HashMap<String, Value>> = Vec::new();

    for i in data {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("name".to_string(), Value::String(i.name));
        map.insert("phone".to_string(), Value::String(i.phone));
        map.insert("email".to_string(), Value::String(i.email));
        map.insert("address".to_string(), Value::String(i.address));
        map.insert("marks".to_string(), json!(i.marks));
        map.insert("percentage".to_string(), json!(i.percentage));
        map.insert("grade".to_string(), json!(i.grade));
        final_data.push(map);
    }

    final_data
}
