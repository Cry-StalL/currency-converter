mod lib;
use lib::get_exchange_rate;
use crate::utils::data_parser::get_currency_list;

mod utils;

fn main() {
    // let price: f64 = get_exchange_rate("USD", "CNY").unwrap();
    // println!("{}", price);
    let list = get_currency_list();
}
