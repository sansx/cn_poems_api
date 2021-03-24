use super::models::ResPoems;
use super::pagination::*;
use super::schema::poems::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::{associations::HasTable, dsl::{Offset, delete, insert_into}, pg::{Pg, types::sql_types}, query_builder::{AsQuery, Query, SqlQuery}, query_dsl::load_dsl};
use diesel::pg::PgConnection;
use diesel::query_dsl::filter_dsl::{FilterDsl, FindDsl};
use diesel::query_dsl::{LoadQuery, RunQueryDsl};
use diesel::query_source::{QuerySource, Queryable, Table};
use diesel::{dsl::Find, query_dsl::methods::OffsetDsl};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

// #[derive(Deserialize, Serialize)]
// struct HttpRes<T> {
//     list: Vec<T>,
//     total: i64,
//     page: i64,
// }

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
    Ok(
        web::block(move || get_pagination_res::<poems, ResPoems>(db, poems, pagination.page))
            .await
            .map(|poem| HttpResponse::Ok().json(poem.unwrap()))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_poems<T, Table>(
    pool: web::Data<Pool>,
    table:Table,
    page: i64,
) -> Result<HttpRes<T>, diesel::result::Error>
where
Paginated<Table>:  LoadQuery<PgConnection, (T, i64)>  ,
Table: OffsetDsl,
Offset<Table>:  LoadQuery<PgConnection, (T, i64)>,
T:diesel::deserialize::Queryable
//     Table: diesel::associations::HasTable,
//     <Table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<diesel::pg::Pg>,
    // Table<T>: Queryable
{
    // let mut scores = HashMap::new();
    let safepage = if page < 1 { 1 } else { page };

    let conn = pool.get().unwrap();
    // let item = table
    //     // .limit(5)
    //     .select("*")
    //     // .offset(0)
    //     .paginate(safepage)
    //     .load_and_count_pages::<(i32, T)>(&conn)?;
    // load::<ResPoems>(&conn)?;
    let (list, total) =table.offset(0).paginate(safepage).load_and_count_pages::<T>(&conn)?;
    // let results = table.load::<(T, i64)>(&conn)?;
    // let total = results.get(0).map(|x| x.1).unwrap_or(0);
    // let records = results.into_iter().map(|x| x.0).collect();
    // let total_pages = (total as f64 / 10 as f64).ceil() as i64;
    // scores.entry(String::from("list")).or_insert(items);
    // scores.entry(String::from("total")).or_insert(num);

    Ok(HttpRes { list, total, page })
}

pub async fn get_poem_by_id() -> impl Responder {
    format!("hello from get users by id")
}

#[derive(Deserialize, Serialize)]
struct HttpRes<T> {
    list: Vec<T>,
    total: i64,
    page: i64,
}

fn get_pagination_res<Table, T>(
    pool: web::Data<Pool>,
    table:Table,
    page: i64,
) -> Result<HttpRes<T>, diesel::result::Error>
where
    Table:  LoadQuery<PgConnection, T>,
    Paginated<Table>:  LoadQuery<PgConnection, (T, i64)>  + RunQueryDsl<PgConnection> ,
    //  load_dsl::LoadQuery<PgConnection, (T, i64)> + diesel::associations::HasTable + RunQueryDsl<PgConnection> ,
    // T:  load_dsl::LoadQuery<PgConnection, T>
{
    let conn = pool.get().unwrap();
    let (list, total) = table.paginate(page).load_and_count_pages::<T>(&conn)?;
    Ok(HttpRes { list, total, page })
}
