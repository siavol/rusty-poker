// use yew::prelude::*;
use yew::{html, Component, Context, Html};
use gloo_net::http::Request;

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
            title: "Story #123".to_string()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_create_session = ctx.link().callback(|_| NewSessionMsg::CreateSession);

        html! {
            <div class="container">
                <div class="form-group">
                    <label for="topic">{ "New session title" }</label>
                    <input type="text" id="topic" name="topic" />
                </div>
                <button class="btn btn_bord u_margin_top" onclick={on_create_session}>{ "Create" }</button>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NewSessionMsg::CreateSession => {
                log::info!("New title: {}", self.title);
                wasm_bindgen_futures::spawn_local(async move {
                    Request::post("/api/session")
                        .header("Content-Type", "application/json")
                        .body("test body")
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