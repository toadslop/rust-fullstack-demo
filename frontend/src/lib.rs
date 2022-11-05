use api::beer::get_beers;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod api;

#[function_component(App)]
fn app() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let beers = get_beers().await;
        gloo_console::log!(beers);
    });
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<App>();
}
