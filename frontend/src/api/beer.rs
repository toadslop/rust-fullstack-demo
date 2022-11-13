use super::get_api_url;
use entity::beer::Model as Beer;
use entity::review::Model as Review;
use reqwasm::http::Request;
use shared::{ApiQueryParams, BEERS_ROUTE};
use std::error::Error;

pub async fn get_beers() -> Result<Vec<Beer>, Box<dyn Error>> {
    let url = get_api_url(BEERS_ROUTE);

    let beers = Request::get(url.as_str())
        .send()
        .await?
        .json::<Vec<Beer>>()
        .await?;

    Ok(beers)
}

pub async fn get_beer(
    id: i32,
    queries: Option<ApiQueryParams>,
) -> Result<(Beer, Vec<Review>), Box<dyn Error>> {
    let mut url = get_api_url(&[BEERS_ROUTE, &id.to_string()].join("/"));

    if let Some(queries) = queries {
        if let Some(expand) = &queries.expand {
            url.set_query(Some(&format!("expand={}", expand)))
        }
    }

    let beer = Request::get(url.as_str())
        .send()
        .await?
        .json::<(Beer, Vec<Review>)>()
        .await?;

    Ok(beer)
}
