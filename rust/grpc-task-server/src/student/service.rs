use tonic::{Response,Request,Status};

use crate::utils::db_config::{get_data,put_data,delete_data};

use self::service::{Id , Resp , StudRequest};

use service::student_server::Student;


pub mod service{
    tonic::include_proto!("services");
}

#[derive(Debug,Default)]
pub struct StudentService{}

#[tonic::async_trait]
impl Student for StudentService{
    async fn get_student (&self , id : Request<Id>) -> Result<Response<Resp> , Status> {
        if let Ok(data) = get_data(format!("s-{}" , id.into_inner().id)).await{
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

    async fn add_student(&self , payload : Request<StudRequest>) -> Result<Response<Resp> , Status>  {
        let inner_payload = payload.into_inner().clone(); // Cloning the payload
        let value = format!(
            "id: {}, name: {}, phone: {}, email: {}, city: {}, address: {}, marks: {:?}",
            inner_payload.id,
            inner_payload.name,
            inner_payload.phone,
            inner_payload.email,
            inner_payload.city,
            inner_payload.address,
            inner_payload.marks
        );   

        if put_data(format!("s-{}" , inner_payload.id.to_string()), value.clone()).await {
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

    async fn delete_student (&self , id : Request<Id>) -> Result<Response<Resp> , Status>  {
        if let Ok(()) = delete_data(format!("s-{}" , id.into_inner().id)).await{
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

    async fn put_student(&self , payload: Request<StudRequest>) -> Result<Response<Resp>, Status> {
        let inner_payload = payload.into_inner().clone(); // Cloning the payload
        if let Ok(data) = get_data(format!("s-{}" , inner_payload.id)).await{
            let value = format!(
                "id: {}, name: {}, phone: {}, email: {}, city: {}, address: {}, marks: {:?}",
                inner_payload.id,
                inner_payload.name,
                inner_payload.phone,
                inner_payload.email,
                inner_payload.city,
                inner_payload.address,
                inner_payload.marks
            ); 

            if put_data(format!("s-{}" , inner_payload.id), value.clone()).await {
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