use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use database::BeerQueries;

use crate::config::app_state::AppState;

pub async fn beers(_req: HttpRequest, data: Data<AppState>) -> impl Responder {
    let db = &data.db;
    let beers = BeerQueries::find_all(db).await;

    match beers {
        Ok(beers) => HttpResponse::Ok().json(beers),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
