use std::{fmt::format, sync::Arc};

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Error;
use uuid::Uuid;

use crate::{
    services::common_st::Message, utils::db_config::{get_data, put_data}, STUDENT
};

use super::model::Student;


pub async fn create(Json(student): Json<Student>) -> Response {
    if let Ok(data) = get_data("student".to_string()).await{
        let student_vec : Result<Vec<Student> , Error> = serde_json::from_str(&data);
        match student_vec {
            Ok(mut data) => {
                data.push(student.clone());
                write_data_into_db(data).await;
                Json(Message {
                    status: 200,
                    message_key: "Student created".to_string(),
                    data: student.clone(),
                })
                .into_response()
            }Err(err) => {
                println!("Err in : {:?}" , err);
                todo!()
            }
        }
    }else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data: student,
        })
        .into_response()
    }
}
pub async fn get_all() -> Response {
    if let Ok(data) = get_data("student".to_string()).await{
        let student_vec : Result<Vec<Student> , Error> = serde_json::from_str(&data);
        match student_vec {
            Ok(data) => {
                Json(Message {
                    status: 200,
                    message_key: "Student created".to_string(),
                    data: data,
                })
                .into_response()
            }Err(err) => {
                println!("Err in : {:?}" , err);
                todo!()
            }
        }
    }else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data:"empty Data".to_string(),
        })
        .into_response()
    }
}
pub async fn update(Path(id): Path<i32>, Json(student): Json<Student>) -> Response {
    if let Ok(data) = get_data("student".to_string()).await{
        let student_vec : Result<Vec<Student> , Error> = serde_json::from_str(&data);
        match student_vec {
            Ok(mut data) => {

                let mut flag = false;

                for i in data.iter_mut(){
                    if i.id == id{
                        i.id = student.id.clone();
                        i.name = student.name.clone();
                        i.address = student.address.clone();
                        i.marks = student.marks.clone();
                        i.phone = student.phone.clone();
                        flag = true;
                        break;
                    }
                }

                if flag {

                    write_data_into_db(data).await;
                    Json(Message {
                        status: 200,
                        message_key: "Student Updated".to_string(),
                        data: student,
                    })
                    .into_response()
                }else {
                    Json(Message {
                        status: 404,
                        message_key: "Student is not available".to_string(),
                        data: "Enter right data".to_string(),
                    })
                    .into_response()
                }
            }Err(err) => {
                println!("Err in : {:?}" , err);
                todo!()
            }
        }
    }else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data:"empty Data".to_string(),
        })
        .into_response()
    }
}

pub async fn get(Path(id): Path<i32>) -> Response {
    if let Ok(data) = get_data("student".to_string()).await{
        let student_vec : Result<Vec<Student> , Error> = serde_json::from_str(&data);
        match student_vec {
            Ok(data) => {
                let mut flag = false;
                let mut student = Student {
                    id: 0, // Initialize with a default value
                    name: String::new(),
                    email: String::new(),
                    phone: String::new(),
                    address: String::new(),
                    marks: vec![], // Initialize marks as an empty vector
                };
                for i in data.clone().into_iter(){
                    if i.id == id{
                        flag = true;
                        student = i;
                    }
                }
                if flag{
                    Json(Message {
                        status: 200,
                        message_key: "Student created".to_string(),
                        data: student,
                    })
                    .into_response()
                }else {
                    Json(Message {
                        status: 404,
                        message_key: "Id is not matched".to_string(),
                        data: "Student is not available",
                    })
                  .into_response()
                }
            }Err(err) => {
                println!("{:?}" , err);
                todo!();
            }
        }
    }else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data:"empty Data".to_string(),
        })
        .into_response()
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    if let Ok(data) = get_data("student".to_string()).await{
        let student_vec : Result<Vec<Student> , Error> = serde_json::from_str(&data);
        match student_vec {
            Ok(mut data) => {
                let mut flag = false;
                data.retain(|e| {
                    if e.id != id {
                        return true;
                    } else {
                        flag = true;
                        return false;
                    }
                });

                if flag {
                    write_data_into_db(data).await;
                    Json(Message {
                        status: 200,
                        message_key: "Success!!".to_string(),
                        data: format!("Student on id {} is deleted" , id),
                    })
                    .into_response()
                }else {
                    Json(Message {
                        status: 404,
                        message_key: "fatal!!".to_string(),
                        data: format!("Student on id {} is not matched" , id),
                    })
                    .into_response()

                }
            }Err(err) => {
                println!("Err in : {:?}" , err);
                todo!()
            }
        }
    }else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data:"empty Data".to_string(),
        })
        .into_response()
    }
}


async fn write_data_into_db(data : Vec<Student>) {
    match serde_json::to_string(&data) {
        Ok(str) => {
            put_data("student".to_string() , str).await;
        }Err(err) => {
            println!("{:?}" , err);
        }
    }
}