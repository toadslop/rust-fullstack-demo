use reqwasm::http::Request;

use super::get_api_url;
use entity::beer::Model as Beer;

pub async fn get_beers() -> Vec<Beer> {
    let url = get_api_url("");

    Request::get(&url)
        .send()
        .await
        .unwrap()
        .json::<Vec<Beer>>()
        .await
        .unwrap()
}
