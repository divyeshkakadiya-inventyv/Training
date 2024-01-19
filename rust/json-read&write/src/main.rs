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

fn main(){
    
}


