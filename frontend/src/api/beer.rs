// use database::env::BEERS_ROUTE;
use reqwasm::http::Request;
use shared::BEERS_ROUTE;

use super::get_api_url;
use entity::beer::Model as Beer;

pub async fn get_beers() -> Vec<Beer> {
    let url = get_api_url(BEERS_ROUTE);

    Request::get(url.as_str())
        .send()
        .await
        .unwrap()
        .json::<Vec<Beer>>()
        .await
        .unwrap()
}
