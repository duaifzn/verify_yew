mod component;
mod dto;
use component::app::App;
use yew;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
