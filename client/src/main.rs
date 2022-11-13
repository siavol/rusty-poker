use yew::{html, Component, Context, Html, NodeRef};
use web_sys::HtmlInputElement;
use gloo_net::http::Request;
use serde_json;
use rusty_poker_common::{NewSessionParams, Session};

enum NewSessionMsg {
    CreateSession,
    UpdateTitle(String)
}

struct NewSessionUI {
    title_input: NodeRef,
    title: String,
}

impl Component for NewSessionUI {
    type Message = NewSessionMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            title_input: NodeRef::default(),
            title: "".to_string()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let title_input_ref = self.title_input.clone();

        let on_create_session = link.callback(|_| NewSessionMsg::CreateSession);
        let on_change_title = link.batch_callback(move |_| {
            let input = title_input_ref.cast::<HtmlInputElement>();
            input.map(|input| NewSessionMsg::UpdateTitle(input.value()))
        });

        html! {
            <div class="container">
                <div class="form-group">
                    <label for="topic">{ "start new session" }</label>
                    <input type="text" id="topic"
                        ref={self.title_input.clone()}
                        name="topic" placeholder="your session title"
                        oninput={on_change_title} />
                </div>
                <button class="btn btn_bord u_margin_top" onclick={on_create_session}>{ "create" }</button>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NewSessionMsg::CreateSession => {
                let params = NewSessionParams{
                    title: self.title.clone()
                };
                log::debug!("Creating new session with params: {:?}", params);
                let request_body = serde_json::to_string(&params).unwrap();
                wasm_bindgen_futures::spawn_local(async move {
                    let new_session: Session = Request::post("/api/session")
                        .header("Content-Type", "application/json")
                        .body(request_body)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    log::debug!("New session created {:?}", new_session);
                });
                false
            },
            NewSessionMsg::UpdateTitle(value) => {
                log::debug!("Entered title value: {}", value);
                self.title = value;
                false
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<NewSessionUI>();
}
