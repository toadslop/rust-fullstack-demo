use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<App>();
}
