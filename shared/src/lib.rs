use serde::Deserialize;

pub const FRONTEND_URL_KEY: &str = "FRONTEND_URL";
pub const DEFAULT_FRONTEND_URL: &str = "http://localhost:8000";
pub const DATABASE_URL_KEY: &str = "DATABASE_URL";
pub const BACKEND_URL_KEY: &str = "BACKEND_URL";
pub const DEFAULT_BACKEND_URL: &str = "http://localhost:8080";
pub const BEERS_ROUTE: &str = "/beers";
pub const SINGLE_BEER_ROUTE: &str = "/beers/{beer_id}";
pub const REVIEWS_BY_BEER_ROUTE: &str = "/beers/{beer_id}/reviews";

#[derive(Deserialize)]
pub struct ApiQueryParams {
    pub expand: Option<String>,
}
