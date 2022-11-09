use super::get_api_url;
use entity::beer::Model as Beer;
use reqwasm::http::Request;
use shared::BEERS_ROUTE;

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

pub async fn get_beer(id: i32) -> Option<Beer> {
    let url = get_api_url(BEERS_ROUTE);

    Request::get(url.as_str())
        .send()
        .await
        .unwrap()
        .json::<Option<Beer>>()
        .await
        .unwrap()
}
