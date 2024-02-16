
use std::string::FromUtf8Error;

use tikv_client::RawClient;

pub async fn get_client() -> RawClient{
    RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap()
}

pub async fn put_data(key:String , value:String) -> bool{
    let client = get_client();
    client.await.put(key, value).await.unwrap();
    true
}

pub async fn get_data(key:String) -> Result<String , FromUtf8Error>{
    let client = get_client();
    match client.await.get(key.clone()).await.unwrap() {
        Some(value) => {
            String::from_utf8(value)
        },
        None => {
            println!("key is not matched : {:?}" , key);
            todo!()
        }
    }
}

pub async fn delete_data(key:String) {
    let client = get_client();
    client.await.delete("1234".to_string()).await.unwrap();
}