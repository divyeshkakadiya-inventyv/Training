
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Error;

use crate::{
    services::common_st::Message, utils::db_config::{get_data, put_data},
};

use super::model::Customer;


/// add the customer
pub async fn create(Json(customer): Json<Customer>) -> Response {
    if let Ok(data) = get_data("Customer".to_string()).await{
        let customer_vec : Result<Vec<Customer> , Error> = serde_json::from_str(&data);
        match customer_vec {
            Ok(mut data) => {
                data.push(customer.clone());
                write_data_into_db(data).await;
                Json(Message {
                    status: 200,
                    message_key: "Customer created".to_string(),
                    data: customer.clone(),
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
            data: customer,
        })
        .into_response()
    }
}

///handler for the get all customer
pub async fn get_all() -> Response {
    if let Ok(data) = get_data("Customer".to_string()).await{
        let customer_vec : Result<Vec<Customer> , Error> = serde_json::from_str(&data);
        match customer_vec {
            Ok(data) => {
                Json(Message {
                    status: 200,
                    message_key: "Customer created".to_string(),
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
pub async fn update(Path(id): Path<i32>, Json(customer): Json<Customer>) -> Response {
    if let Ok(data) = get_data("Customer".to_string()).await{
        let customer_vec : Result<Vec<Customer> , Error> = serde_json::from_str(&data);
        match customer_vec {
            Ok(mut data) => {

                let mut flag = false;
                for i in data.iter_mut(){
                    if i.id == id{
                        i.name = customer.name.clone();
                        i.skills = customer.skills.clone();
                        i.language = customer.language.clone();
                        i.status = customer.status.clone();
                        flag = true;
                        break;
                    }
                }

                if flag {

                    write_data_into_db(data).await;
                    Json(Message {
                        status: 200,
                        message_key: "Customer Updated".to_string(),
                        data: customer,
                    })
                    .into_response()
                }else {
                    Json(Message {
                        status: 404,
                        message_key: "Customer is not available".to_string(),
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
    if let Ok(data) = get_data("Customer".to_string()).await{
        let customer_vec : Result<Vec<Customer> , Error> = serde_json::from_str(&data);
        match customer_vec {
            Ok(data) => {
                let mut flag = false;
                let mut customer = Customer {
                    id: 0, // Initialize with a default value
                    name: String::new(),
                    skills : vec![],
                    status: String::new(),
                    language: String::new(),
                    // Initialize marks as an empty vector
                };
                for i in data.clone().into_iter(){
                    if i.id == id{
                        flag = true;
                        customer = i;
                    }
                }
                if flag{
                    Json(Message {
                        status: 200,
                        message_key: "Customer is here".to_string(),
                        data: customer,
                    })
                    .into_response()
                }else {
                    Json(Message {
                        status: 404,
                        message_key: "Id is not matched".to_string(),
                        data: "Customer is not available",
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
    if let Ok(data) = get_data("Customer".to_string()).await{
        let customer_vec : Result<Vec<Customer> , Error> = serde_json::from_str(&data);
        match customer_vec {
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
                        data: format!("Customer on id {} is deleted" , id),
                    })
                    .into_response()
                }else {
                    Json(Message {
                        status: 404,
                        message_key: "fatal!!".to_string(),
                        data: format!("Customer on id {} is not matched" , id),
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


async fn write_data_into_db(data : Vec<Customer>) {
    match serde_json::to_string(&data) {
        Ok(str) => {
            put_data("Customer".to_string() , str).await;
        }Err(err) => {
            println!("{:?}" , err);
        }
    }
}