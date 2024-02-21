use tonic::{Response,Request,Status};

use crate::utils::db_config::{get_data,put_data,delete_data};

use self::service::{Id , Resp , CusRequest};

use service::customer_server::Customer;


pub mod service{
    tonic::include_proto!("services");
}

#[derive(Debug,Default)]
pub struct CustomerService{}

#[tonic::async_trait]
impl Customer for CustomerService{
    async fn get_customer (&self , id : Request<Id>) -> Result<Response<Resp> , Status> {
        if let Ok(data) = get_data(format!("c-{}" , id.into_inner().id)).await{
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

    async fn add_customer(&self , payload : Request<CusRequest>) -> Result<Response<Resp> , Status>  {
        let inner_payload = payload.into_inner().clone(); // Cloning the payload
        let value = format!(
            "id: {}, name: {}, skills: {:?}, status: {}, language: {}",
            inner_payload.id,
            inner_payload.name,
            inner_payload.skills,
            inner_payload.status,
            inner_payload.language
        );  

        if put_data(format!("c-{}" , inner_payload.id.to_string()), value.clone()).await {
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

    async fn delete_customer (&self , id : Request<Id>) -> Result<Response<Resp> , Status>  {
        if let Ok(()) = delete_data(format!("c-{}" , id.into_inner().id)).await{
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

    async fn put_customer(&self , payload: Request<CusRequest>) -> Result<Response<Resp>, Status> {
        let inner_payload = payload.into_inner().clone(); // Cloning the payload
        if let Ok(data) = get_data(format!("c-{}" , inner_payload.id)).await{
            let value = format!(
                "id: {}, name: {}, skills: {:?}, status: {}, language: {}",
                inner_payload.id,
                inner_payload.name,
                inner_payload.skills,
                inner_payload.status,
                inner_payload.language
            );

            if put_data(format!("c-{}" , inner_payload.id), value.clone()).await {
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