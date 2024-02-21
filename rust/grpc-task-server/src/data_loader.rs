use std::fs;

use serde_json::Error;

use crate::{customer::model::Customer, employee::model::Empl, student::model::Student, utils::db_config::put_data};

///data is loaded into the databases
pub async fn emp_loader() {
    match fs::read_to_string("./src/files/Employee.json"){
        Ok(value) => {
            let data : Result<Vec<Empl> , Error> = serde_json::from_str(&value);
            match data{
                Ok(data) => {
                    for i in data {
                        let value = format!(
                            "id: {}, name: {}, age: {}, skills: {:?}, position: {:?}, experience_years: {:?}",
                            i.id,
                            i.name,
                            i.age,
                            i.skills,
                            i.position,
                            i.experience
                        ); 
                        put_data(format!("e-{}" , i.id) , value).await;
                    }
                },
                Err(err) => todo!("{}" , err),
            }
        }Err(err) => todo!("{}" , err),
    }
}

pub async fn stud_loader(){
    match fs::read_to_string("./src/files/StudentData.json"){
        Ok(value) => {
            let data : Result<Vec<Student> , Error> = serde_json::from_str(&value);
            match data{
                Ok(data) => {
                    for i in data {
                        let value = format!(
                            "id: {}, name: {}, phone: {}, email: {}, address: {}, marks: {:?}",
                            i.id,
                            i.name,
                            i.phone,
                            i.email,
                            i.address,
                            i.marks
                        );   
                        put_data(format!("s-{}" , i.id) , value).await;
                    }
                },
                Err(err) => todo!("{}" , err),
            }
        }Err(err) => todo!("{}" , err),
    }
}

pub async fn cust_loader(){
    match fs::read_to_string("./src/files/Master_Data.json"){
        Ok(value) => {
            let data : Result<Vec<Customer> , Error> = serde_json::from_str(&value);
            match data{
                Ok(data) => {
                    for i in data {
                        let value = format!(
                            "id: {}, name: {}, skills: {:?}, status: {}, language: {}",
                            i.id,
                            i.name,
                            i.skills,
                            i.status,
                            i.language
                        );  
                        put_data(format!("c-{}" , i.id) , value).await;
                    }
                },
                Err(err) => todo!("{}" , err),
            }
        }Err(err) => todo!("{}" , err),
    }
}