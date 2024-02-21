pub mod utils;
pub mod student;
pub mod employee;
pub mod customer;
pub mod data_loader;

use crate::student::service::StudentService;
use crate::student::service::service::student_server::StudentServer;

pub mod service {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main(){
    let addr = "127.0.0.1:8080".parse().unwrap();
    let studentservice = StudentService::default();

    println!("Server is running on http://127.0.0.1:8080");

    tonic::transport::Server::builder()
        .add_service(StudentServer::new(studentservice))
        .serve(addr)
        .await.unwrap();
}
