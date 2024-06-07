use currency_converter::get_exchange_rate;

fn main() {
    let price: f64 = get_exchange_rate("USD", "CNY").unwrap();
    println!("{}", price);
}
