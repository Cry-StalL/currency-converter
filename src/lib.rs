mod back;

use yew::prelude::*;
use stylist::css;
use crate::back::backend::get_exchange_rate;

#[function_component]
pub fn App() -> Html { // function components can't be async
    // let style = css!("color: red; font-size: 20px;");
    let exchange_rate = get_exchange_rate("USD","CNY");

    html! {
        <div>
            <p>{exchange_rate}</p>
        </div>
    }
}