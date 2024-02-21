use tonic::{Response,Request,Status};

use crate::utils::db_config::{get_data,put_data,delete_data};

use self::service::{Id , Resp , EmpRequest};

use service::employee_server::Employee;


pub mod service{
    tonic::include_proto!("services");
}

#[derive(Debug,Default)]
pub struct EmployeeService{}

#[tonic::async_trait]
impl Employee for EmployeeService{
    async fn get_employee (&self , id : Request<Id>) -> Result<Response<Resp> , Status> {
        if let Ok(data) = get_data(format!("e-{}" , id.into_inner().id)).await{
            Ok(Response::new(
                Resp {
                    status : 200,
                    message : "Id is matched!".to_string(),
                    data : data
                }
            ))
        }else {
            Ok(Response::new(
                Resp {
                    status : 404,
                    message : "Id is not matched!".to_string(),
                    data : "".to_string()
                }
            ))
        }
    }

    async fn add_employee(&self , payload : Request<EmpRequest>) -> Result<Response<Resp> , Status>  {
        let inner_payload = payload.into_inner().clone(); // Cloning the payload
        let value = format!(
            "id: {}, name: {}, age: {}, skills: {:?}, position: {}, experience_years: {}",
            inner_payload.id,
            inner_payload.name,
            inner_payload.age,
            inner_payload.skills,
            inner_payload.position,
            inner_payload.experience_years
        );  

        if put_data(format!("e-{}" , inner_payload.id.to_string()), value.clone()).await {
            Ok(Response::new(
                Resp {
                    status : 200,
                    message : "data is added".to_string(),
                    data : value
                }
            ))
        }else {
            Ok(Response::new(
                Resp {
                    status : 500,
                    message : "Internal Server Error".to_string(),
                    data : "".to_string()
                }
            ))
        }
    }

    async fn delete_employee (&self , id : Request<Id>) -> Result<Response<Resp> , Status>  {
        if let Ok(()) = delete_data(format!("e-{}" , id.into_inner().id)).await{
            Ok(Response::new(
                Resp {
                    status : 200,
                    message : "Id is matched!".to_string(),
                    data : "data is deleted successfully!!".to_string()
                }
            ))
        }else {
            Ok(Response::new(
                Resp {
                    status : 404,
                    message : "Id is not matched!".to_string(),
                    data : "".to_string()
                }
            ))
        }
    }

    async fn put_employee(&self , payload: Request<EmpRequest>) -> Result<Response<Resp>, Status> {
        let inner_payload = payload.into_inner().clone(); // Cloning the payload
        if let Ok(data) = get_data(format!("e-{}" , inner_payload.id)).await{
            let value = format!(
                "id: {}, name: {}, age: {}, skills: {:?}, position: {}, experience_years: {}",
                inner_payload.id,
                inner_payload.name,
                inner_payload.age,
                inner_payload.skills,
                inner_payload.position,
                inner_payload.experience_years
            );

            if put_data(format!("e-{}" , inner_payload.id), value.clone()).await {
                Ok(Response::new(
                    Resp {
                        status : 200,
                        message : "Id is matched!".to_string(),
                        data : value.clone()
                    }
                ))
            }else {
                Ok(Response::new(
                    Resp {
                        status : 500,
                        message : "Internal Server Error".to_string(),
                        data : "".to_string()
                    }
                ))
            }
            
        }else {
            Ok(Response::new(
                Resp {
                    status : 404,
                    message : "Id is not matched!".to_string(),
                    data : "User not found for Update".to_string()
                }
            ))
        }
        
    }
}