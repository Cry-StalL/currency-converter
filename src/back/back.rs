// use reqwest;
// use tokio;
// use serde_json::{Value};
//
// #[tokio::main]
// pub async fn get_exchange_rate(currency_from: &str, currency_to: &str) -> Result<f64, Box<dyn std::error::Error>>{
//     let url = format!("https://finance.pae.baidu.com/vapi/v1/getquotation?group=huilv_minute&need_reverse_real=1&code={currency_from}{currency_to}&finClientType=pc");
//
//     let response = reqwest::get(url).await?;
//     let content = response.text().await?;
//     let content_json: Value = serde_json::from_str(&content)?;
//
//     let exchange_rate: f64 = content_json["Result"]["cur"]["price"].as_str().unwrap().replace("\"", "").parse::<f64>().unwrap();
//
//     Ok(exchange_rate)
// }
