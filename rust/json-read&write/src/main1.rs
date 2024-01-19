use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug , Deserialize , Serialize)]
struct Student{
    name:String,
    phone:String,
    email:String,
    address:String,
    marks:Vec<f32>,
    pecentage : Option<f32>,
    grade : Option<String>
}

impl Student{

    //calculate percentages from the marks
    fn calculate_percentage(&mut self) -> f32{
        let sum = self.marks.clone().into_iter().reduce(|a,b| a + b);
        match sum {
            Some(sum) => {
                (sum / self.marks.len() as f32) as f32
            }
            None => 0.0
        }
    }

    //assign grade according to the percentages
    fn grade(&mut self) -> String{
        match self.pecentage {
            Some(percentage) => {
                if percentage>70.0 {
                    "A".to_string()
                }else if percentage>60.0 && percentage<70.0 {
                    "B".to_string()
                }else if percentage>40.0 && percentage<60.0 {
                    "C".to_string()
                }else{
                    "D".to_string()
                }
            }
            None => "Error is generated in grade Desiding" . to_string()
        }
    }
}

fn main() {
     // Read the contents in a string
     let json_string = fs::read_to_string("./assets/StudentData.json").expect("Could't Perform Operation");

    println!("{:?}" , json_string);

    //convert data into the vector of structure
    let student_data : Vec<Student>  = serde_json::from_str(&json_string).expect("Failed to deserialize."); 

    println!("{:#?}",student_data); //# is use to display pritty json format

    // add percentage into all structs 
    let updated_data : Vec<Student>= student_data.into_iter().map(|mut data|  { 
        data.pecentage = Some(data.calculate_percentage()); 
        data.grade = Some(data.grade());
        data
    }).collect();

    println!("{:#?}" , updated_data);
    

    //convert again into the string and write it in a new file
    let final_data = serde_json::to_string_pretty(&updated_data).expect("Data is not converted in the json");
    fs::write("./assets/Calculated_data" , final_data).expect("data is not write in file");

}

