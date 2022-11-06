use load_dotenv::load_dotenv;
pub mod beer;

pub fn get_api_url(endpoint: &str) -> String {
    load_dotenv!();
    let base_url = env!("BACKEND_URL");
    [base_url, endpoint].join("/")
}
