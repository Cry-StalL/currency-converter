
use reqwest::Client;
use std::{error::Error as StdError, fmt::{self, Error}, time::Duration};
use serde_json::Value;

#[derive(Debug)]
enum MyError {
    From_Type_Error(String),
    To_Type_Error(String),
    Type_Error(String),
    FA_Error(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
           
            MyError::From_Type_Error(ref err) => write!(f, "From_Type_Error : {}", err),
            MyError::To_Type_Error(ref err) => write!(f, "To_Type_Error : {}", err),
            MyError::FA_Error(ref err) => write!(f, "Amount_Error : {}", err),
            MyError::Type_Error(ref err) => write!(f, "Types Error: {}", err),
        }
    }
}


impl StdError for MyError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            MyError::From_Type_Error(_) => None,
            MyError::To_Type_Error(_) => None,
            MyError::FA_Error(_) => None,
            MyError::Type_Error(_) => None,
        }
    }
}

pub async fn get_exchange_rate(currency_from: &str, currency_to: &str) -> Result<f64, Box<dyn StdError>>{
    let client = Client::builder()
        // .timeout(Duration::from_secs(10))  // 设置超时值
        .build()?;

    let url = format!("https://api.freecurrencyapi.com/v1/latest?apikey=fca_live_0iH3VzeURHPkxOfBl7t0xSsMzfg9kywq8ESfwSLJ&currencies={currency_to}&base_currency={currency_from}");
    let response = client.get(url).send().await?;

    let content = response.text().await?;
    let content_json: Value = serde_json::from_str(&content).unwrap();
    match &content_json["errors"] {
        Value::Null => {
            web_sys::console::log_1(&"successfully get rate".into());
            let exchange_rate = content_json["data"][currency_to].as_f64().unwrap();

            return Ok(exchange_rate);
        },
        Value::Bool(_) => todo!(),
        Value::Number(_) => todo!(),
        Value::String(_) => todo!(),
        Value::Array(_) => todo!(),
        Value::Object(Errormap) => {
            web_sys::console::log_1(&"Error".into());
            web_sys::console::log_1(&content.into());
            
            // match map["base_currency"]{
            //     Value::Null => {
            //         web_sys::console::log_1(&"To type Error".into());
            //         return Err(Box::new(MyError::Type_Error("Invaild To Currency Type".to_string())));
            //     },
            //     Value::Bool(_) => todo!(),
            //     Value::Number(_) => todo!(),
            //     Value::String(_) => todo!(),
            //     Value::Array(_) => {
            //         web_sys::console::log_1(&"From type Error".into());
            //         return Err(Box::new(MyError::Type_Error("Invaild From Currency Type".to_string())));
            //     },
            //     Value::Object(_) => todo!(),
            // }
            return Err(Box::new(MyError::Type_Error("Invaild currency type".to_string())));
        }
            
    }
    // web_sys::console::log_1(&content.into());
    // let exchange_rate = content_json["data"][currency_to].as_f64().unwrap_or(0.0);

    // return Ok(exchange_rate);
}
//
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