

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Error;

use crate::{
    services::common_st::Message,
    utils::db_config::{get_data, put_data},
    Empl,
};

pub async fn create(Json(Empl): Json<Empl>) -> Response {
    if let Ok(data) = get_data("Empl".to_string()).await {
        let empl_vec: Result<Vec<Empl>, Error> = serde_json::from_str(&data);
        match empl_vec {
            Ok(mut data) => {
                data.push(Empl.clone());
                write_data_into_db(data).await;
                Json(Message {
                    status: 200,
                    message_key: "Employee created".to_string(),
                    data: Empl.clone(),
                })
                .into_response()
            }
            Err(err) => {
                println!("Err in : {:?}", err);
                todo!()
            }
        }
    } else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data: Empl,
        })
        .into_response()
    }
}
pub async fn get_all() -> Response {
    if let Ok(data) = get_data("Empl".to_string()).await {
        let empl_vec: Result<Vec<Empl>, Error> = serde_json::from_str(&data);
        match empl_vec {
            Ok(data) => Json(Message {
                status: 200,
                message_key: "Employee created".to_string(),
                data: data,
            })
            .into_response(),
            Err(err) => {
                println!("Err in : {:?}", err);
                todo!()
            }
        }
    } else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data: "empty Data".to_string(),
        })
        .into_response()
    }
}
pub async fn update(Path(id): Path<i32>, Json(empl): Json<Empl>) -> Response {
    if let Ok(data) = get_data("Empl".to_string()).await {
        let empl_vec: Result<Vec<Empl>, Error> = serde_json::from_str(&data);
        match empl_vec {
            Ok(mut data) => {
                let mut flag = false;

                for i in data.iter_mut() {
                    if i.id == id {
                        i.name = empl.name.clone();
                        i.experience = empl.experience.clone();
                        i.position = empl.position.clone();
                        i.age = empl.age.clone();
                        i.skills = empl.skills.clone();
                        flag = true;
                        break;
                    }
                }

                if flag {
                    write_data_into_db(data).await;
                    Json(Message {
                        status: 200,
                        message_key: "Employee Updated".to_string(),
                        data: empl,
                    })
                    .into_response()
                } else {
                    Json(Message {
                        status: 404,
                        message_key: "Employee is not available".to_string(),
                        data: "Enter right data".to_string(),
                    })
                    .into_response()
                }
            }
            Err(err) => {
                println!("Err in : {:?}", err);
                todo!()
            }
        }
    } else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data: "empty Data".to_string(),
        })
        .into_response()
    }
}

pub async fn get(Path(id): Path<i32>) -> Response {
    if let Ok(data) = get_data("Empl".to_string()).await {
        let empl_vec: Result<Vec<Empl>, Error> = serde_json::from_str(&data);
        match empl_vec {
            Ok(data) => {
                let mut flag = false;
                let mut empl = Empl {
                    id: 0, // Initialize with a default value
                    name: String::new(),
                    experience: Some(0),
                    position: Some(String::new()),
                    age: 0,
                    skills: vec![],
                };
                for i in data.clone().into_iter() {
                    if i.id == id {
                        flag = true;
                        empl = i;
                    }
                }
                if flag {
                    Json(Message {
                        status: 200,
                        message_key: "Employee created".to_string(),
                        data: empl,
                    })
                    .into_response()
                } else {
                    Json(Message {
                        status: 404,
                        message_key: "Id is not matched".to_string(),
                        data: "Employee is not available",
                    })
                    .into_response()
                }
            }
            Err(err) => {
                println!("{:?}", err);
                todo!();
            }
        }
    } else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data: "empty Data".to_string(),
        })
        .into_response()
    }
}

pub async fn delete(Path(id): Path<i32>) -> Response {
    if let Ok(data) = get_data("Empl".to_string()).await {
        let empl_vec: Result<Vec<Empl>, Error> = serde_json::from_str(&data);
        match empl_vec {
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
                        data: format!("Employee on id {} is deleted", id),
                    })
                    .into_response()
                } else {
                    Json(Message {
                        status: 404,
                        message_key: "fatal!!".to_string(),
                        data: format!("Employee on id {} is not matched", id),
                    })
                    .into_response()
                }
            }
            Err(err) => {
                println!("Err in : {:?}", err);
                todo!()
            }
        }
    } else {
        Json(Message {
            status: 500,
            message_key: "Internal Server Error".to_string(),
            data: "empty Data".to_string(),
        })
        .into_response()
    }
}

async fn write_data_into_db(data: Vec<Empl>) {
    match serde_json::to_string(&data) {
        Ok(str) => {
            put_data("Empl".to_string(), str).await;
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
