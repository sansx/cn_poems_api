use super::models::ResPoems;
use super::pagination::*;
use super::schema::poems::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::dsl::Find;
use diesel::dsl::{delete, insert_into};
use diesel::pg::PgConnection;
use diesel::query_dsl::filter_dsl::{FilterDsl, FindDsl};
use diesel::query_dsl::{LoadQuery, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Deserialize, Serialize)]
struct HttpRes<T> {
    list: Vec<T>,
    total: i64,
    page: i64,
}

#[derive(Deserialize)]
pub struct ProductSearch {
    pub search: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductPagination {
    pub page: i64,
}

pub async fn get_poems(
    db: web::Data<Pool>,
    pagination: web::Query<ProductPagination>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", pagination);
    Ok(web::block(move || {
        get_all_poems::<super::schema::poems::table, ResPoems>(db, poems, pagination.page)
    })
    .await
    .map(|poem| HttpResponse::Ok().json(poem.unwrap()))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_poems<Table, T>(
    pool: web::Data<Pool>,
    table: Table,
    page: i64,
) -> Result<HttpRes<ResPoems>, diesel::result::Error>
where
    Table: FindDsl<i32> + FilterDsl<i32>,
    Find<Table, i32>: LoadQuery<PgConnection, T>,
{
    // let mut scores = HashMap::new();
    let safepage = if page < 1 { 1 } else { page };

    let conn = pool.get().unwrap();
    let (list, total) = table
        // .limit(5)
        // .offset(0)
        .paginate(safepage)
        .load_and_count_pages::<T>(&conn)?;
    // load::<ResPoems>(&conn)?;

    // scores.entry(String::from("list")).or_insert(items);
    // scores.entry(String::from("total")).or_insert(num);

    Ok(HttpRes { list, total, page })
}

pub async fn get_poem_by_id() -> impl Responder {
    format!("hello from get users by id")
}
