use crate::{
    dto::verify_data::{VerifyDataDto},
    component::{input_tokenid::InputTokenId, verify_table::VerifyTable},
};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};

#[function_component(App)]
pub fn app() -> Html {
    let fetch_verify_data = use_state(|| None);
    let fetch_verify_data_clone = fetch_verify_data.clone();
    let token_id: UseStateHandle<String> = use_state(|| "".to_string());

    let search_click = {
        let fetch_verify_data = fetch_verify_data.clone();
        let token_id = token_id.clone();
        Callback::from(move |_|{
            let fetch_verify_data = fetch_verify_data.clone();
            let token_id = token_id.clone();
            spawn_local(async move {
                let fetch_videos: VerifyDataDto =
                    Request::get(&format!("https://iprooftest.com/api/token/free/{}", (*token_id).clone()))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                fetch_verify_data.set(Some(fetch_videos));
            });
        })
    };

    let on_input = {
        let token_id = token_id.clone();
        Callback::from(move |e: InputEvent|{
            let token_id = token_id.clone();
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            // web_sys::console::log_1(&target.value().into());
            token_id.set(target.value());
        })
    };

    html! {
            <div>
                <h1>{"Verify Data"}</h1>
                <InputTokenId token_id={(*token_id).clone()} on_input={on_input} on_click={search_click}/>
                if (*fetch_verify_data_clone).is_some(){
                    <VerifyTable verify_data={(*fetch_verify_data_clone).clone()}/>
                }
            </div>
        }
}
