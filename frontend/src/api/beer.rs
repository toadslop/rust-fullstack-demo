use std::error::Error;

use super::get_api_url;
use entity::beer::Model as Beer;
use reqwasm::http::Request;
use shared::BEERS_ROUTE;

pub async fn get_beers() -> Result<Vec<Beer>, Box<dyn Error>> {
    let url = get_api_url(BEERS_ROUTE);

    let beers = Request::get(url.as_str())
        .send()
        .await?
        .json::<Vec<Beer>>()
        .await?;

    Ok(beers)
}

pub async fn get_beer(id: i32) -> Result<Beer, Box<dyn Error>> {
    let url = get_api_url(&[BEERS_ROUTE, &id.to_string()].join("/"));

    let beer = Request::get(url.as_str())
        .send()
        .await?
        .json::<Beer>()
        .await?;

    Ok(beer)
}
