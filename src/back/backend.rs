
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












//
// pub fn handle_error(err: Box<dyn StdError>) {
//     if let Some(reqwest_err) = err.downcast_ref::<reqwest::Error>() {
//         handle_error1(reqwest_err);
//     } else if let Some(parse_err) = err.downcast_ref::<std::num::ParseFloatError>() {
//         handle_error2(parse_err);
//     } else {
//         eprintln!("An unexpected error occurred: {}", err);
//     }
// }
//
// pub fn handle_error1(e: &reqwest::Error) {
//     if e.is_builder(){//应该不会发生
//         eprintln!("RequestBuilder Err.{}",e);
//     } else if e.is_timeout() {
//         eprintln!("Request timed out.{}",e);
//     } else if e.is_connect() {
//         eprintln!("Failed to connect.{}",e);
//     } else if e.is_decode() {
//         eprintln!("Failed to decode the response body.{}",e);
//     } else if e.is_status() {
//         if let Some(status) = e.status() {
//             eprintln!("HTTP error with status code: {}", status);
//         } else {
//             eprintln!("HTTP error with unknown status code.{}",e);
//         }
//     } else if e.is_request() {//应该不会发生
//         eprintln!("Request error: {}", e);
//     } else {
//         eprintln!("An unexpected error occurred: {}", e);
//     }
// }
// pub fn handle_error2(e:&std::num::ParseFloatError){
//     println!("There might no be such currency as one of the input.");
// }

// fn handle_jsonerror(err: serde_json::Error) {
//     match err.classify() {
//         serde_json::error::Category::Io => {
//             eprintln!("I/O Error: {}", err);
//         }
//         serde_json::error::Category::Syntax => {
//             eprintln!("Syntax Error: {}", err);
//         }
//         serde_json::error::Category::Data => {
//             eprintln!("Data Error: {}", err);
//         }
//         serde_json::error::Category::Eof => {
//             eprintln!("EOF Error: {}", err);
//         }
//     }
// }