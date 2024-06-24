use std::fs;
use serde_json::{Value, json, from_str};

pub fn get_currency_list() -> serde_json::Value {
    let file_path = "data/currency_list.json";
    let json_data = fs::read_to_string(file_path).unwrap(); //TODO
    println!("{}", json_data);
    let mut data: Value = from_str(&json_data).expect("wrong json format");
    data
}