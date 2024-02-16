use std::fs;
use uuid::Uuid;

use crate::utils::db_config::put_data;

use super::{middlewares::get_middlewares, routes::get_routes};


///start the rest_api service and load the data into the db
pub async fn start_rest_api_server(){
    let app = get_middlewares(get_routes());
    load_emp_data().await;
    load_customer_data().await;
    load_student_data().await;
    println!("Server is runing on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///load employee data into the db
async fn load_emp_data(){
    match fs::read_to_string("./src/files/Employee.json"){
        Ok(value) => {
            let id = "Empl".to_string();
            put_data(id, value).await;
        }Err(err) => todo!("{}" , err),
    }
}

///load student data into the db
async fn load_student_data(){
    match fs::read_to_string("./src/files/StudentData.json"){
        Ok(value) => {
            let id = "student".to_string();
            put_data(id, value).await;
        }Err(err) => todo!("{}" , err),
    }
}

///load customer data into the db
async fn load_customer_data(){
    match fs::read_to_string("./src/files/task_assigner/Master_Data.json"){
        Ok(value) => {
            let id = "Customer".to_string();
            put_data(id, value).await;
        }Err(err) => todo!("{}" , err),
    }
}
