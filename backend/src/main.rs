use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use config::{app_state::AppState, cors::get_cors_config, database::get_db_config};
use routes::{add_review::add_review, all_beers::beers, one_beer_by_id::one_beer_by_id};
use shared::{
    BACKEND_URL_KEY, BEERS_ROUTE, DEFAULT_BACKEND_URL, REVIEWS_BY_BEER_ROUTE, SINGLE_BEER_ROUTE,
};
use std::io::Error;

pub mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();

    let db = get_db_config()
        .await
        .map_err(|db_err| Error::new(std::io::ErrorKind::ConnectionAborted, db_err.to_string()))?;

    let state = AppState { db };
    let app_url = std::env::var(BACKEND_URL_KEY).unwrap_or_else(|_| {
        log::info!(
            "Environment variable {} not found. Using default: {}",
            BACKEND_URL_KEY,
            DEFAULT_BACKEND_URL
        );

        String::from(DEFAULT_BACKEND_URL)
    });

    let host_port = app_url.split("//").nth(1).unwrap();

    HttpServer::new(move || {
        let cors = get_cors_config();

        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(cors)
            .route(BEERS_ROUTE, web::get().to(beers))
            .route(SINGLE_BEER_ROUTE, web::get().to(one_beer_by_id))
            .route(REVIEWS_BY_BEER_ROUTE, web::post().to(add_review))
    })
    .bind(host_port)?
    .run()
    .await
}
