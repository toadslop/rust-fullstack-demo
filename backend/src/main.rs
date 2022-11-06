use actix_web::{get, web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder};
use config::{
    app_state::AppState,
    cors::get_cors_config,
    database::get_db_config,
    env_keys::{BACKEND_URL, DEFAULT_BACKEND_URL},
};
use database::BeerQueries;
use std::io::Error;

mod config;

#[get("/")]
async fn hello(_req: HttpRequest, data: Data<AppState>) -> impl Responder {
    let db = &data.db;
    let beers = BeerQueries::find_all(db).await;

    match beers {
        Ok(beers) => HttpResponse::Ok().json(beers),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();

    let db = get_db_config()
        .await
        .map_err(|db_err| Error::new(std::io::ErrorKind::ConnectionAborted, db_err.to_string()))?;

    let state = AppState { db };
    let app_url = std::env::var(BACKEND_URL).unwrap_or_else(|_| {
        log::info!(
            "Environment variable {} not found. Using default: {}",
            BACKEND_URL,
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
            .service(hello)
    })
    .bind(host_port)?
    .run()
    .await
}
