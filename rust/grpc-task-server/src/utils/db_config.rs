
use std::{string::FromUtf8Error};

use tikv_client::{Error, RawClient};

pub async fn get_client() -> RawClient{
    RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap()
}

pub async fn put_data(key:String , value:String) -> bool{
    let client = get_client();
    client.await.put(key, value).await.unwrap();
    true
}

pub async fn get_data(key:String) -> Result<String , String>{
    let client = get_client().await;
    match client.get(key.clone()).await.unwrap() {
        Some(value) => Ok(String::from_utf8(value).unwrap()),
        None => Err("id is not matched!!".to_string())
    }
}

pub async fn delete_data(key:String) -> Result<() , Error>{
    let client = get_client().await;
    match client.delete(key).await {
        Ok(()) => Ok(()),
        Err(err) => Err(err)
    }
}