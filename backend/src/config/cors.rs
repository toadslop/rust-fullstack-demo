use super::env_keys::{DEFAULT_FRONTEND_URL, FRONTEND_URL};
use actix_cors::Cors;
use actix_web::http;

pub fn get_cors_config() -> Cors {
    let frontend_url = &std::env::var(FRONTEND_URL).unwrap_or_else(|_| {
        log::info!(
            "Environment variable {} not found. Using default: {}",
            FRONTEND_URL,
            DEFAULT_FRONTEND_URL
        );

        String::from(DEFAULT_FRONTEND_URL)
    });

    Cors::default()
        .allowed_origin(frontend_url)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
