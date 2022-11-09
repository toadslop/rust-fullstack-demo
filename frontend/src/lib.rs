use routes::{switch, Route};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{function_component, html};
use yew_router::{BrowserRouter, Switch};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod api;
mod components;
pub mod pages;
mod routes;

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<RateBeer>();
}

#[function_component(RateBeer)]
pub fn rate_beer() -> Html {
    html! {
        <div class="container mt-5">
          <h1>{ "Rate Beer" }</h1>
          <div>
            <BrowserRouter>
              <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
          </div>
        </div>
    }
}
