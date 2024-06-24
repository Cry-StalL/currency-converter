use yew::prelude::*;
use stylist::css;

#[function_component]
pub fn App() -> Html {
    let style = css!("color: red; font-size: 20px;");
    html! {
        <>
            <div class={style}>
                {"Hello World!"}
            </div>
            <div>
                <button type="button">{"Click me!"}</button>
            </div>
        </>
    }
}