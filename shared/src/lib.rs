use serde::Deserialize;

pub const DEFAULT_FRONTEND_URL: &str = "http://localhost:8000";
pub const FRONTEND_HOST_KEY: &str = "FRONTEND_HOST";
pub const FRONTEND_PORT_KEY: &str = "FRONTEND_PORT";
pub const FRONTEND_PROTOCOL_KEY: &str = "FRONTEND_PROTOCOL";

pub const DEFAULT_DB_URL: &str = "sqlite:./sqlite.db?mode=rwc";
pub const DATABASE_URL_KEY: &str = "DATABASE_URL";
pub const DATABASE_PROTOCOL_KEY: &str = "DATABASE_PROTOCOL";
pub const DATABASE_PORT_KEY: &str = "DATABASE_PORT";
pub const DATABASE_USER_KEY: &str = "POSTGRES_USER";
pub const DATABASE_PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
pub const DATABASE_NAME_KEY: &str = "POSTGRES_DB";
pub const DATABASE_HOST_KEY: &str = "POSTGRES_HOST";

pub const DEFAULT_BACKEND_URL: &str = "http://localhost:8080";
pub const BACKEND_HOST_KEY: &str = "BACKEND_HOST";
pub const BACKEND_PORT_KEY: &str = "BACKEND_PORT";
pub const BACKEND_PROTOCOL_KEY: &str = "BACKEND_PROTOCOL";

pub const BEERS_ROUTE: &str = "/beers";
pub const SINGLE_BEER_ROUTE: &str = "/beers/{beer_id}";
pub const REVIEWS_BY_BEER_ROUTE: &str = "/beers/{beer_id}/reviews";

#[derive(Deserialize)]
pub struct ApiQueryParams {
    pub expand: Option<String>,
}

pub fn init_database_url() -> Url {
    let mut url = Url::parse(DEFAULT_DB_URL).expect("the default db url to be parseable");

    if let Ok(protocol) = std::env::var(DATABASE_PROTOCOL_KEY) {
        url.set_scheme(&protocol).unwrap();
    }

    if let Ok(protocol) = std::env::var(DATABASE_HOST_KEY) {
        url.set_host(Some(&protocol)).unwrap();
    }

    if let Ok(user) = std::env::var(DATABASE_USER_KEY) {
        url.set_username(&user).unwrap();
    }

    if let Ok(password) = std::env::var(DATABASE_PASSWORD_KEY) {
        url.set_password(Some(&password)).unwrap();
    }

    if let Ok(port) = std::env::var(DATABASE_PORT_KEY) {
        let port = port
            .parse::<u16>()
            .expect("the database port to parseable to u16");
        url.set_port(Some(port)).unwrap();
    };
    println!("{}", url);
    if let Ok(name) = std::env::var(DATABASE_NAME_KEY) {
        url.set_path(&name);
    }

    url
}

pub use url::Url;
