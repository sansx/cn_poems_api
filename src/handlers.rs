use super::errors;
use super::models::*;
use super::pagination::*;
use super::schema::*;
use super::Pool;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::{
    expression::AsExpression,
    pg::{Pg, PgConnection},
    query_builder::{AsQuery, QueryFragment},
    Expression,
};
use diesel::{prelude::*, sql_types::Integer};
use diesel::{
    // dsl::{self, Find, Nullable},
    query_dsl::{
        // methods::{FilterDsl, FindDsl},
        LoadQuery,
        RunQueryDsl,
    },
    sql_types::{Bool, Text},
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, vec::Vec};

#[derive(Deserialize, Serialize)]
struct HttpRes<T> {
    list: Vec<T>,
    total: i64,
    page: i64,
}

#[derive(Deserialize)]
pub struct SearchID {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WriterSearch {
    pub id: Option<i32>,
    pub name: Option<String>,
    // pub keyword: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PoemsSearch {
    pub writer: Option<String>,
    pub dynasty: Option<String>,
    pub keyword: Option<String>,
}

enum SearchRes {
    WRes(String),
    DRes(String),
    KRes(String),
    NRes,
}

impl PoemsSearch {
    fn get_res(self) -> SearchRes {
        let PoemsSearch {
            writer,
            dynasty,
            keyword,
        } = self;
        if writer.is_some() {
            return SearchRes::WRes(writer.unwrap());
        }
        if dynasty.is_some() {
            return SearchRes::DRes(dynasty.unwrap());
        }
        if keyword.is_some() {
            return SearchRes::KRes(keyword.unwrap());
        }
        SearchRes::NRes
    }
}

#[derive(Deserialize, Debug)]
pub struct ProductPagination {
    pub page: i64,
}

pub async fn get_poems(
    db: web::Data<Pool>,
    pagination: Option<web::Query<ProductPagination>>,
) -> Result<HttpResponse, Error> {
    use self::poems::dsl::*;
    println!("{:?}", pagination);
    let page = match pagination {
        None => 1,
        Some(i) => i.page,
    };
    Ok(
        web::block(move || get_pagination_res_tb::<poems, ResPoems>(db, poems, page))
            .await
            .map(|poem| HttpResponse::Ok().json(poem.unwrap()))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn easy_get(pool: web::Data<Pool>, page: i64) -> Result<HttpRes<ResPoems>, diesel::result::Error> {
    use self::poems::dsl::*;
    let safepage = if page < 1 { 1 } else { page };
    let conn = pool.get().unwrap();
    let (list, total) = poems
        .paginate(safepage)
        .load_and_count_pages::<ResPoems>(&conn)?;
    Ok(HttpRes {
        list,
        total,
        page: 1,
    })
}

pub async fn get_poem_by_id(
    db: web::Data<Pool>,
    res: web::Query<SearchID>,
) -> Result<HttpResponse, Error> {
    use super::schema::poems::dsl::{id, poems};
    Ok(
        web::block(move || poems.find(res.id).first::<ResPoems>(&db.get().unwrap()))
            .await
            .map(|poem| HttpResponse::Ok().json(poem.unwrap()))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn get_poems_by_search(
    db: web::Data<Pool>,
    res: Option<web::Query<PoemsSearch>>,
    pagination: Option<web::Query<ProductPagination>>,
) -> Result<HttpResponse, Error> {
    use super::schema::poems::dsl::*;
    let page = match pagination {
        None => 1,
        Some(i) => i.page,
    };
    Ok(
        web::block(move || match res.unwrap().into_inner().get_res() {
            SearchRes::WRes(x) => poems
                .filter(writer.eq(x))
                .paginate(page)
                .load_and_count_pages::<ResPoems>(&db.get().unwrap()),
            SearchRes::DRes(x) => poems
                .filter(dynasty.eq(x))
                .paginate(page)
                .load_and_count_pages::<ResPoems>(&db.get().unwrap()),
            SearchRes::KRes(x) => {
                let re = format!("%{}%", x);
                poems
                    .filter(
                        writer
                            .like(re.clone())
                            .or(title.like(re.clone()))
                            .or(content.like(re.clone())),
                    )
                    .paginate(page)
                    .load_and_count_pages::<ResPoems>(&db.get().unwrap())
            }
            SearchRes::NRes => poems
                .paginate(page)
                .load_and_count_pages::<ResPoems>(&db.get().unwrap()),
        })
        .await
        .map(|poem| {
            let (list, total) = poem.unwrap();
            HttpResponse::Ok().json(HttpRes { list, total, page })
        })
        .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn get_writer_by_search(
    db: web::Data<Pool>,
    res: Option<web::Query<WriterSearch>>,
) -> Result<HttpResponse, Error> {
    use super::schema::authors::dsl::*;
    // if res.is_none() {
    //     return Err(actix_web::error::ErrorNotFound("asd"));
    // }

    Ok(web::block(move || {
      let re = res.unwrap().into_inner();
        if let Some(x) = re.id {
          return  authors.find(x).first::<ResAuthor>(&db.get().unwrap())
        }
        authors
            .filter(name.eq(re.name))
            .first::<ResAuthor>(&db.get().unwrap())
    })
    .await
    .map(|writer| HttpResponse::Ok().json(writer.unwrap()))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_pagination_res_tb<Table, T>(
    pool: web::Data<Pool>,
    table: Table,
    page: i64,
) -> Result<HttpRes<T>, diesel::result::Error>
where
    Paginated<<Table as diesel::query_builder::AsQuery>::Query>:
        LoadQuery<PgConnection, (T, i64)> + RunQueryDsl<PgConnection> + QueryFragment<Pg>,
    Table: AsQuery + Sized + TbPaginate,
    T: Debug,
{
    let conn = pool.get().unwrap();
    let (list, total) = table.paginate(page).load_and_count_pages::<T>(&conn)?;
    Ok(HttpRes { list, total, page })
}

fn get_pagination_res<Table, T>(
    pool: web::Data<Pool>,
    table: Table,
    page: i64,
) -> Result<HttpRes<T>, diesel::result::Error>
where
    Paginated<Table>:
        LoadQuery<PgConnection, (T, i64)> + RunQueryDsl<PgConnection> + QueryFragment<Pg>,
    Table: AsQuery + Sized + Paginate,
    T: Debug,
{
    let conn = pool.get().unwrap();
    let (list, total) = table.paginate(page).load_and_count_pages::<T>(&conn)?;
    Ok(HttpRes { list, total, page })
}

// fn get_pagination_res1<Table, T, U>(
//     pool: web::Data<Pool>,
//     table: Table,
//     page: i64,
//     search: Option<U>,
// ) -> Result<HttpRes<T>, diesel::result::Error>
// where
//     Paginated<<Table as diesel::query_builder::AsQuery>::Query>:
//         LoadQuery<PgConnection, (T, i64)> + RunQueryDsl<PgConnection> + QueryFragment<Pg>,
//     Table: AsQuery + Sized + FilterDsl<U>, // + FindDsl<i64>
//     U: diesel::expression::Expression + AsExpression<Text>,
//     <Table as FilterDsl<U>>::Output: Paginate,
//     Paginated<<<Table as FilterDsl<U>>::Output as diesel::query_builder::AsQuery>::Query>:
//         LoadQuery<diesel::PgConnection, (T, i64)>,
//     // diesel::query_builder::SelectStatement<Table>: FilterDsl<dyn diesel::expression::Expression<SqlType = Text>>,
//     // diesel::query_builder::SelectStatement<Table>: FilterDsl<U>
//     // Find<Table, i64>: LoadQuery<PgConnection, Paginated<Table>>,
//     // <Table as FindDsl<i64>>::Output: crate::pagination::Paginate,
//     // Paginated<<<Table as FindDsl<i64>>::Output as diesel::query_builder::AsQuery>::Query>:
//     //     LoadQuery<PgConnection, (T, i64)> + RunQueryDsl<PgConnection> + QueryFragment<Pg>,
//     //  load_dsl::LoadQuery<PgConnection, (T, i64)> + diesel::associations::HasTable + RunQueryDsl<PgConnection> ,
//     // T:  load_dsl::LoadQuery<PgConnection, T>
// {
//     let conn = pool.get().unwrap();
//     let (list, total) = table
//         .filter(search.unwrap())
//         .paginate(page)
//         .load_and_count_pages::<T>(&conn)?;
//     Ok(HttpRes { list, total, page })
// }
