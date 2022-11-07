use load_dotenv::load_dotenv;
use pages::beer_list::BeerList;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{function_component, html};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod api;
mod components;
mod pages;

#[wasm_bindgen]
pub fn main() {
    load_dotenv!();

    yew::start_app::<RateBeer>();
}

#[function_component(RateBeer)]
pub fn rate_beer() -> Html {
    html! {
        <div class="container mt-5">
          <h1>{ "Rate Beer" }</h1>
          <div>
            <BeerList />
          </div>
        </div>
    }
}
