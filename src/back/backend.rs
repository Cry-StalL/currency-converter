
use reqwest::Client;
use std::{error::Error as StdError, fmt::{self}};
use serde_json::Value;

#[derive(Debug)]
pub enum MyError {
    NetworkError(String),
    TypeError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::TypeError(msg) => write!(f, "Type error: {}", msg),
            MyError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}


impl StdError for MyError {}


pub async fn get_exchange_rate(currency_from: &str, currency_to: &str) -> Result<f64, Box<dyn StdError>>{
    let client = Client::builder()
        .build()?;
    let url = format!("https://api.freecurrencyapi.com/v1/latest?apikey=fca_live_0iH3VzeURHPkxOfBl7t0xSsMzfg9kywq8ESfwSLJ&currencies={currency_to}&base_currency={currency_from}");
    let response = match client.get(url).send().await{
        Ok(r) => r,
        Err(e) => {
            return Err(Box::new(MyError::NetworkError("Get response failed :".to_string()+e.to_string().as_str())));
        },
    };
    let content = response.text().await?;
    let content_json: Value = serde_json::from_str(&content).unwrap();
    match &content_json["errors"] {
        Value::Null => {
            web_sys::console::log_1(&"successfully get rate".into());
            let exchange_rate = content_json["data"][currency_to].as_f64().unwrap();
            return Ok(exchange_rate);
        },
        _ => {            
            return Err(Box::new(MyError::TypeError("Invalid currency type".to_string())));
        }       
    }
}

pub fn cal(currency_from: f64 , rate:f64) -> f64{
    let currency_to= currency_from * rate;
    currency_to
}

