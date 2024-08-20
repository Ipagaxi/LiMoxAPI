
#[cfg(target_os = "linux")]
use std::os::linux::raw::stat;

use reqwest::{self, Error};
use tauri::AppHandle;
use serde_json::{Value};

use crate::{IP_ADDRESS, PASSWORD, PORT, USERNAME};

pub async fn test_lrs_connection(app_handle: AppHandle) -> (bool, String) {
    let client = reqwest::Client::new();
    let (ip, port, username, password) = unsafe {
        let ip = IP_ADDRESS.clone();
        let port = PORT.clone();
        let username = USERNAME.clone();
        let password = PASSWORD.clone();
        (ip, port, username, password)
    };

    let url = format!("http://{}:{}/xapi/statements", ip, port);  
    let response = client.get(&url)
        .header("Content-Type", "application/json")
        .header("X-Experience-API-Version", "1.0.3")
        .basic_auth(username, Some(password))
        .send()
        .await;
    
    match response {
        Ok(res) => {
            let body = res.text().await.unwrap();
            match serde_json::from_str::<Value>(&body) {
                Ok(_) => {
                    (true, body)
                },
                Err(_) => {
                    (false, body)
                }
            }
        },
        Err(e) => {
            println!("Request failed with error: {}", e);
            (false, e.to_string())
        }
    }
}

pub async fn request_statements_since(timestamp: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let (ip, port, username, password) = unsafe {
        let ip = IP_ADDRESS.clone();
        let port = PORT.clone();
        let username = USERNAME.clone();
        let password = PASSWORD.clone();
        (ip, port, username, password)
    };

    let url = format!("http://{}:{}/xapi/statements?since={}", ip, port, timestamp);  
    let response = client.get(&url)
        .header("Content-Type", "application/json")
        .header("X-Experience-API-Version", "1.0.3")
        .basic_auth(username, Some(password))
        .send()
        .await;
    
    match response {
        Ok(res) => {
            let body = res.text().await?;
            //println!("Request successful");
            Ok(body) // Return success
        },
        Err(e) => {
            println!("Request failed with error: {}", e);
            Err(e)
        }
    }
}