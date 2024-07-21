mod back;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::back::backend::get_exchange_rate;
use crate::back::backend::cal;
use wasm_bindgen_futures::spawn_local;
use back::backend::MyError;
use std::error::Error as StdError;


pub struct MyComponent {
    input_value_from: String,
    input_value_from_amount: f64,
    input_value_to: String,
    convert_result:f64,
    prompt : bool,
    neterr : bool,
}

pub enum Msg {
    AsyncResult(Result<f64, Box<dyn StdError>>),
    CallAsyncFunction,
    UpdateInputF(String),
    UpdateInputFA(f64),
    UpdateInputT(String),
    Promptclose
}


impl Component for MyComponent {

    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        Self {
            
            input_value_from: String::new(),
            input_value_from_amount: 0.0,
            input_value_to: String::new(),
            convert_result: 0.0,
            prompt : false,
            neterr : false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AsyncResult(result) => {
                match result {
                    Ok(rate) => {
                        // 处理成功的情况，例如更新组件的状态
                        web_sys::console::log_1(&"Async function returned: ".into());
                        let Result = cal(self.input_value_from_amount,rate);
                        self.convert_result = Result;
                    }
                    Err(err) => {
                        match err.downcast_ref::<MyError>(){
                            Some(MyError::TypeError(_)) => {
                                self.prompt = true
                            }
                            Some(MyError::NetworkError(_)) => {
                                self.neterr = true
                            }
                            None => {
                                web_sys::console::log_1(&"An unknown error occurred while getting rate:".into());
                                web_sys::console::log_1(&err.to_string().into());
                                // 处理其他类型的错误
                            }
                        }
                    }
                }
                true // 返回 true 表示需要重新渲染组件
            }
            Msg::CallAsyncFunction => {
                let callback = ctx.link().callback(Msg::AsyncResult);
                let from =  self.input_value_from.clone();
                let to = self.input_value_to.clone();
                spawn_local(async move {
                    let rate = get_exchange_rate(from.as_str(), to.as_str()).await;
                    callback.emit(rate);
                });
                false
            },
            Msg::UpdateInputF(value) => {
                web_sys::console::log_1(&"UpdateInputF".into());
                self.input_value_from = value;
                true
            }
            Msg::UpdateInputFA(value) => {
                web_sys::console::log_1(&"UpdateInputFA".into());
                self.input_value_from_amount = value;
                true
            }
            Msg::UpdateInputT(value) => {
                web_sys::console::log_1(&"UpdateInputT".into());
                self.input_value_to = value;
                true
            },
            Msg::Promptclose => {
                self.prompt = false;
                self.neterr = false;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInputF(input.value())
        });
        let oninput2 = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse::<f64>().unwrap_or(0.0);
            Msg::UpdateInputFA(value)
        });
        let oninput3 = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInputT(input.value())
        });

        if self.prompt{
            html! {
                <body class="layout">
    
                    <div>
                        <h1 class = "HEAD">
                            {"本网页为您提供方便快捷地货币换算服务"}
                        </h1>
                    </div>
                    
                    <div class="modal-overlay" >
                        <div class="modal-content" >
                            { "请输入正确的货币缩写"}
                            <button onclick={ctx.link().callback(|_| Msg::Promptclose)}>{ "x" }</button>
                        </div>
                    </div>

                    <div class="container">
                        <div class="login-group">
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput} value={self.input_value_from.clone()} id = ""  type="text"  placeholder=" "/>
                                <label class="float-placeholder">
                                    {"From"}
                                </label>
                            </div>
                            <br/>
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput2} type = "number" id = "From_Amount" placeholder=" "/>
                                <label class="float-placeholder">
                                    {"Amount"}
                                </label>
                            </div>
                            <br/>
                        </div>
                        <div class="arrow-right"></div>
                        <div class="login-group">
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput3} value={self.input_value_to.clone()} id = ""  type="text"  placeholder=" "/>
                                <label class="float-placeholder">
                                    {"To"}
                                </label>
                            </div>
                            <br/>
                            <br/>
                            <div class="float-input-group">
                                <input value={self.convert_result.to_string()} id = "To_Amount" type="text" placeholder=" "/>
                                <label class="float-placeholder">
                                    {"Amount"}
                                </label>
                            </div>
                            <br/>
                        </div>
                    </div>
    
                    <div class="container">
                        <button class="ConvertBUT" onclick={ctx.link().callback(|_| Msg::CallAsyncFunction)} >
                            <span>{"转换"}</span>
                        </button>
                    </div>
                    
                    <div>
                        <h1 class = "BUTTOM">
                            {"—————————————————————————————————————————————————————————————————————————————————————————"}
                        </h1>
                    </div>
    
                </body>
            }
        } 
        else if self.neterr {
            html! {
                <body class="layout">
    
                    <div>
                        <h1 class = "HEAD">
                            {"本网页为您提供方便快捷地货币换算服务"}
                        </h1>
                    </div>
                    
                    <div class="modal-overlay" >
                        <div class="modal-content" >
                            { "请检查网络"}
                            <button onclick={ctx.link().callback(|_| Msg::Promptclose)}>{ "x" }</button>
                        </div>
                    </div>

                    <div class="container">
                        <div class="login-group">
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput} value={self.input_value_from.clone()} id = ""  type="text"  placeholder=" "/>
                                <label class="float-placeholder">
                                    {"From"}
                                </label>
                            </div>
                            <br/>
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput2} type = "number" id = "From_Amount" placeholder=" "/>
                                <label class="float-placeholder">
                                    {"Amount"}
                                </label>
                            </div>
                            <br/>
                        </div>
                        <div class="arrow-right"></div>
                        <div class="login-group">
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput3} value={self.input_value_to.clone()} id = ""  type="text"  placeholder=" "/>
                                <label class="float-placeholder">
                                    {"To"}
                                </label>
                            </div>
                            <br/>
                            <br/>
                            <div class="float-input-group">
                                <input value={self.convert_result.to_string()} id = "To_Amount" type="text" placeholder=" "/>
                                <label class="float-placeholder">
                                    {"Amount"}
                                </label>
                            </div>
                            <br/>
                        </div>
                    </div>
    
                    <div class="container">
                        <button class="ConvertBUT" onclick={ctx.link().callback(|_| Msg::CallAsyncFunction)} >
                            <span>{"转换"}</span>
                        </button>
                    </div>
                    
                    <div>
                        <h1 class = "BUTTOM">
                            {"—————————————————————————————————————————————————————————————————————————————————————————"}
                        </h1>
                    </div>
    
                </body>
            }
        }
        else {
            html! {
                <body class="layout">
    
                    <div>
                        <h1 class = "HEAD">
                            {"本网页为您提供方便快捷地货币换算服务"}
                        </h1>
                    </div>
                    
    
                    <div class="container">
                        <div class="login-group">
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput} value={self.input_value_from.clone()} id = ""  type="text"  placeholder=" "/>
                                <label class="float-placeholder">
                                    {"From"}
                                </label>
                            </div>
                            <br/>
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput2} type = "number" id = "From_Amount" placeholder=" "/>
                                <label class="float-placeholder">
                                    {"Amount"}
                                </label>
                            </div>
                            <br/>
                        </div>
                        <div class="arrow-right"></div>
                        <div class="login-group">
                            <br/>
                            <div class="float-input-group">
                                <input oninput={oninput3} value={self.input_value_to.clone()} id = ""  type="text"  placeholder=" "/>
                                <label class="float-placeholder">
                                    {"To"}
                                </label>
                            </div>
                            <br/>
                            <br/>
                            <div class="float-input-group">
                                <input value={self.convert_result.to_string()} id = "To_Amount" type="text" placeholder=" "/>
                                <label class="float-placeholder">
                                    {"Amount"}
                                </label>
                            </div>
                            <br/>
                        </div>
                    </div>
    
                    <div class="container">
                        <button class="ConvertBUT" onclick={ctx.link().callback(|_| Msg::CallAsyncFunction)} >
                            <span>{"转换"}</span>
                        </button>
                    </div>
                    
                    <div>
                        <h1 class = "BUTTOM">
                            {"—————————————————————————————————————————————————————————————————————————————————————————"}
                        </h1>
                    </div>
    
                </body>
            }
        }
        

    }
}


