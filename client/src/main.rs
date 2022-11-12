// use yew::prelude::*;
use yew::{html, Component, Context, Html};
use gloo_net::http::Request;
use serde_json;
use rusty_poker_common::{NewSessionParams};

enum NewSessionMsg {
    CreateSession
}

struct NewSessionUI {
    title: String,
}

impl Component for NewSessionUI {
    type Message = NewSessionMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            title: "".to_string()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_create_session = ctx.link().callback(|_| NewSessionMsg::CreateSession);

        html! {
            <div class="container">
                <div class="form-group">
                    <label for="topic">{ "start new session" }</label>
                    <input type="text" id="topic" name="topic" placeholder="your session title" />
                </div>
                <button class="btn btn_bord u_margin_top" onclick={on_create_session}>{ "create" }</button>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NewSessionMsg::CreateSession => {
                log::info!("New title: {}", self.title);
                let session = NewSessionParams{
                    title: "test session".to_string()
                };
                let request_body = serde_json::to_string(&session).unwrap();
                wasm_bindgen_futures::spawn_local(async move {
                    Request::post("/api/session")
                        .header("Content-Type", "application/json")
                        .body(request_body)
                        .send()
                        .await
                        // .unwrap()
                        // .json()
                        // .await
                        .unwrap();
                });
                false
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<NewSessionUI>();
}
