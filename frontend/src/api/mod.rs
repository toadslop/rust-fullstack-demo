use load_dotenv::load_dotenv;
use shared::{Url, DEFAULT_BACKEND_URL};
pub mod beer;
pub mod review;

pub fn get_api_url(endpoint: &str) -> Url {
    load_dotenv!();

    let mut default_backend =
        Url::parse(DEFAULT_BACKEND_URL).expect("the default backend url to be parseable");

    let backend_port = env!("BACKEND_PORT")
        .parse::<u16>()
        .expect("the backend port to be parseable");

    default_backend.set_port(Some(backend_port)).unwrap();

    let backend_host = env!("BACKEND_HOST");

    default_backend.set_host(Some(backend_host)).unwrap();

    let backend_protocol = env!("BACKEND_PROTOCOL");

    default_backend.set_scheme(backend_protocol).unwrap();

    default_backend
        .join(endpoint)
        .expect("the base url and the endpoint to be joinable")
}
