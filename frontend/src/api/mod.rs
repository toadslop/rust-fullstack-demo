// use database::env::BEERS_ROUTE;
use load_dotenv::load_dotenv;

use url::Url;
pub mod beer;

pub fn get_api_url(endpoint: &str) -> Url {
    load_dotenv!();
    let base_url = env!("BACKEND_URL");
    let base_url = Url::parse(base_url).expect("a valid url to have been passed from ");
    let full_url = base_url
        .join(endpoint)
        .expect("the base url and the endpoint to be joinable");
    full_url
}
