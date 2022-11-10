use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::BeerQueries;

use crate::config::app_state::AppState;

pub async fn one_beer_by_id(path: web::Path<i32>, data: Data<AppState>) -> impl Responder {
    let beer_id = path.into_inner();
    let db = &data.db;
    let beer = BeerQueries::find_one(db, beer_id).await;

    match beer {
        Ok(beer) => match beer {
            Some(beer) => HttpResponse::Ok().json(beer),
            None => HttpResponse::NotFound().json(format!("Beer with id {} not found", beer_id)),
        },
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}
