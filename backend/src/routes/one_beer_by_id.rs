use crate::config::app_state::AppState;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use database::{sea_orm::DbErr, BeerQueries};
use entity::beer::Relation;
use shared::ApiQueryParams;
use std::str::FromStr;

pub async fn one_beer_by_id(
    path: web::Path<i32>,
    data: Data<AppState>,
    queries: web::Query<ApiQueryParams>,
) -> impl Responder {
    let id = path.into_inner();
    let db = &data.db;

    if let Some(expand) = &queries.expand {
        let relation = Relation::from_str(expand);
        match relation {
            Ok(relation) => match BeerQueries::find_with_related(db, id, relation).await {
                Ok(beer) => match &beer.0 {
                    Some(_) => HttpResponse::Ok().json(beer),
                    None => handle_not_found(id),
                },
                Err(err) => handle_internal_error(err),
            },
            Err(_) => handle_bad_request(BadRequestType::InvalidRelation(expand)),
        }
    } else {
        match BeerQueries::find_one(db, id).await {
            Ok(beer) => match beer {
                Some(beer) => HttpResponse::Ok().json(beer),
                None => handle_not_found(id),
            },
            Err(err) => handle_internal_error(err),
        }
    }
}

fn handle_internal_error(err: DbErr) -> HttpResponse {
    HttpResponse::InternalServerError().json(err.to_string())
}

fn handle_not_found(id: i32) -> HttpResponse {
    HttpResponse::NotFound().json(format!("Beer with id {} not found", id))
}

enum BadRequestType<'a> {
    InvalidRelation(&'a String),
}

fn handle_bad_request(request_type: BadRequestType) -> HttpResponse {
    match request_type {
        BadRequestType::InvalidRelation(relation) => HttpResponse::BadRequest().json(format!(
            "{} is not a valid relation for the requested entity",
            relation
        )),
    }
}
