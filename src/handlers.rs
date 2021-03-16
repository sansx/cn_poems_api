use super::models::ResPoems;
use super::schema::poems::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

pub async fn get_poems(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_poems(db))
        .await
        .map(|poem| HttpResponse::Ok().json(poem.unwrap()))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_poems(pool: web::Data<Pool>) -> Result<Vec<ResPoems>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = poems.limit(5).offset(5).load::<ResPoems>(&conn)?;
    Ok(items)
}

pub async fn get_poem_by_id() -> impl Responder {
    format!("hello from get users by id")
}

