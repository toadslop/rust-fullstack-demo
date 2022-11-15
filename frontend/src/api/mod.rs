use load_dotenv::load_dotenv;
use url::Url;
pub mod beer;
pub mod review;

pub fn get_api_url(endpoint: &str) -> Url {
    load_dotenv!();
    let base_url = env!("BACKEND_URL");
    let base_url = Url::parse(base_url).expect("a valid url to have been passed from ");
    base_url
        .join(endpoint)
        .expect("the base url and the endpoint to be joinable")
}
