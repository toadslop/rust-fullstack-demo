use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::BeerQueries;
use entity::review::Model as Review;

use crate::config::app_state::AppState;

pub async fn add_review(
    path: web::Path<i32>,
    data: Data<AppState>,
    rating: web::Json<Review>,
) -> impl Responder {
    let beer_id = path.into_inner();
    let db = &data.db;
    let new_review = BeerQueries::add_review(db, beer_id, rating.0).await;

    match new_review {
        Ok(review) => HttpResponse::Created().json(review),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
