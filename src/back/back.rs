// use std::f64;
// use std::time::Duration;
// use reqwest;
// use reqwest::Client;
// use std::error::Error as StdError;
// use serde_json;
// use tokio;
// use serde_json::Value;
//
// #[tokio::main]
// pub async fn get_exchange_rate(currency_from: &str, currency_to: &str) -> Result<f64, Box<dyn StdError>>{
//     let client = Client::builder()
//         .timeout(Duration::from_secs(10))  // 设置超时值
//         .build()?;
//
//     let url = format!("https://finance.pae.baidu.com/vapi/v1/getquotation?group=huilv_minute&need_reverse_real=1&code={currency_from}{currency_to}&finClientType=pc");
//     let response = client.get(url).send().await?;
//
//     let content = response.text().await?;
//     let content_json: Value = serde_json::from_str(&content).unwrap();
//     let exchange_rate = content_json["Result"]["cur"]["price"].as_str().unwrap().parse::<f64>()?;
//     return Ok(exchange_rate);
// }
//
// pub fn cul(currency_from: f64 , rate:f64) -> f64{
//     let currency_to= currency_from * rate;
//     currency_to
// }
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
//     } else if e.is_status() {//应该不会发生
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