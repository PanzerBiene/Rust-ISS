use reqwest;
use serde_json::{Value};

#[tokio::main]
async fn main() {
    get_pos().await;
}

async fn get_pos()  {
    let url = "http://api.open-notify.org/iss-now.json";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    let output: Value;
    match response {
        Ok(json) => {
           output = serde_json::from_str(&json).unwrap(); 
           println!("{:?}", output)
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }


}
