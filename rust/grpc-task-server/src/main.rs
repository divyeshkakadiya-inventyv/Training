pub mod utils;
pub mod student;
pub mod employee;
pub mod customer;
pub mod data_loader;

use crate::employee::service::EmployeeService;
use crate::employee::service::service::employee_server::EmployeeServer;
use crate::customer::service::CustomerService;
use crate::customer::service::service::customer_server::CustomerServer;
use crate::student::service::StudentService;
use crate::student::service::service::student_server::StudentServer;
use data_loader::*;

pub mod service {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main(){
    stud_loader().await; cust_loader().await; emp_loader().await;
    let addr = "127.0.0.1:8080".parse().unwrap();
    let studentservice = StudentService::default();
    let employeeservice = EmployeeService::default();
    let customerservice = CustomerService::default();

    println!("Server is running on http://127.0.0.1:8080");

    tonic::transport::Server::builder()
        .add_service(StudentServer::new(studentservice))
        .add_service(EmployeeServer::new(employeeservice))
        .add_service(CustomerServer::new(customerservice))
        .serve(addr)
        .await.unwrap();
}
