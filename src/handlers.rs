use super::models::*;
use super::pagination::*;
use super::schema::{authors, poems};
use super::Pool;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::{
    dsl::Find,
    query_dsl::{methods::FindDsl, LoadQuery, RunQueryDsl},
    QueryDsl,
};
use diesel::{
    pg::{Pg, PgConnection},
    query_builder::{AsQuery, QueryFragment},
};
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
    pagination: Option<web::Query<ProductPagination>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", pagination);
    let page = match pagination {
        None => 1,
        Some(i) => i.page,
    };
    // poems::
    Ok(web::block(move || {
        get_pagination_res::<authors::dsl::authors, ResAuthor>(db, authors::dsl::authors, page)
    })
    .await
    .map(|poem| HttpResponse::Ok().json(poem.unwrap()))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

// fn easy_get(pool: web::Data<Pool>,page: i64,) -> Result<HttpRes<ResPoems>, diesel::result::Error> {
//   let safepage = if page < 1 { 1 } else { page };
//   let conn = pool.get().unwrap();
//   let (list, total) = poems.paginate(safepage).load_and_count_pages::<ResPoems>(&conn)?;
//   Ok(HttpRes { list, total, page })
// }

pub async fn get_poem_by_id() -> impl Responder {
    format!("hello from get users by id")
}

fn get_pagination_res<Table, T>(
    pool: web::Data<Pool>,
    table: Table,
    page: i64,
) -> Result<HttpRes<T>, diesel::result::Error>
where
    Paginated<<Table as diesel::query_builder::AsQuery>::Query>:
        LoadQuery<PgConnection, (T, i64)> + RunQueryDsl<PgConnection> + QueryFragment<Pg>,
    Table: AsQuery + Sized + FindDsl<i64>,
    Find<Table, i64>: LoadQuery<PgConnection, Paginated<Table>>,
    <Table as FindDsl<i64>>::Output: crate::pagination::Paginate,
    Paginated<<<Table as FindDsl<i64>>::Output as diesel::query_builder::AsQuery>::Query>:
        LoadQuery<PgConnection, (T, i64)> + RunQueryDsl<PgConnection> + QueryFragment<Pg>,
    //  load_dsl::LoadQuery<PgConnection, (T, i64)> + diesel::associations::HasTable + RunQueryDsl<PgConnection> ,
    // T:  load_dsl::LoadQuery<PgConnection, T>
{
    let conn = pool.get().unwrap();
    let (list, total) = table
        .find(1)
        .paginate(page)
        .load_and_count_pages::<T>(&conn)?;
    Ok(HttpRes { list, total, page })
}
