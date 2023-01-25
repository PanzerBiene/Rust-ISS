use reqwest;
use serde_json::{Value};
use std::{thread, time};

// TODO
// seperate matching response into seperate function
// add colored output
// get people by space craft
// allow user to specify a spacecraft
// get closest country (maybe)

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut pos: Value;
    let mut i = 5;
    //set sleep_duration to 1 second
    let sleep_duration = time::Duration::from_millis(1000);

    let people = get_people(&client).await;
    println!("number of people: {:?}", people["number"].as_i64().unwrap());
    while i > 0 {
        //get position
        pos = get_pos(&client).await;
        println!("Latitude: {:?} Longitude: {:?}", pos["iss_position"]["latitude"].as_str().unwrap(), pos["iss_position"]["longitude"].as_str().unwrap());
        i-=1;
        //sleep for 1 second
        thread::sleep(sleep_duration);
    }
}

//gets the value returned from calling the open-notify api
async fn get_pos(client: &reqwest::Client) -> Value {
    let url = "http://api.open-notify.org/iss-now.json";
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
        }
        Err(e) => {
            output = Value::Null;
            println!("{:?}", e);
        }
    }

    return output;

}

async fn get_people(client: &reqwest::Client) -> Value {
    let url = "http://api.open-notify.org/astros.json";
    let response = client.get(url)
        .send()
        .await.unwrap()
        .text()
        .await;

    let output: Value;
    match response {
        Ok(json) => {
            output = serde_json::from_str(&json).unwrap();
        }
        Err(e) => {
            output = Value::Null;
            println!("{:?}", e);
        }
    }
    return output;
}
