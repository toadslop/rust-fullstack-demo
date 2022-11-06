use dotenv::dotenv;
use load_dotenv::load_dotenv;
use pages::beer_list::BeerList;
use wasm_bindgen::prelude::wasm_bindgen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod api;
mod components;
mod pages;

#[wasm_bindgen]
pub fn main() {
    load_dotenv!();
    yew::start_app::<BeerList>();
}
