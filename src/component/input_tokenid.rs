use yew::prelude::*;
use web_sys::InputEvent;

#[derive(Clone, Properties, PartialEq)]
pub struct TokenProps {
    pub token_id: String,
    pub on_input: Callback<InputEvent>,
    pub on_click: Callback<String>,
}

#[function_component(InputTokenId)]
pub fn input_tokenid(TokenProps { token_id, on_input, on_click }: &TokenProps) ->Html{
    let on_button_click = {
            let on_click = on_click.clone();
            let token_id = token_id.clone();
            Callback::from(move |_| {
                let token_id = token_id.clone();
                on_click.emit(token_id.clone());
            })
    };
    html! {
        <div>
            <input placeholder={"token_id"} oninput={on_input}/>
            <button onclick={on_button_click.clone()}>
                { "Search" }
            </button>
        </div>
    }
}