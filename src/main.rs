use reqwest;
use serde_json::{Value};
use std::{thread, time};
use colored::*;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut pos: Value;
    //set sleep_duration to 1 second
    let sleep_duration = time::Duration::from_millis(1000);
    
    println!("\n\n{}","========================================================".red());
    println!("{}", "Welcome to Rust-ISS prec crtl + c any time to quit".red());
    println!("{}","========================================================".red());

    let people = get_people(&client).await["number"].as_i64().unwrap();

    println!("{} {}", "number of people:".cyan(), people.to_string().green());
    println!("{} {}", "people in space:".cyan(), get_people(&client).await["people"].to_string().yellow());

    loop {
        //get position
        pos = get_pos(&client).await;
        println!("{} {} {} {}", "Latitude:".cyan(), "Longitude:".cyan(), pos["iss_position"]["latitude"].as_str().unwrap().green(), pos["iss_position"]["longitude"].as_str().unwrap().green());
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

    return match_response(response);

}

async fn get_people(client: &reqwest::Client) -> Value {
    let url = "http://api.open-notify.org/astros.json";
    let response = client.get(url)
        .send()
        .await.unwrap()
        .text()
        .await;

    return match_response(response);
}

//matches response Result
fn match_response(response: Result<String, reqwest::Error>)-> Value {
    let output: Value;
    match response {
        Ok(json) => {
            output = serde_json::from_str(&json).unwrap();
        }
        Err(e) => {
            output = Value::Null;
            println!("{}", e);
        }
    }
    return output;
}
