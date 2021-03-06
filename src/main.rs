#[macro_use]
extern crate diesel;

extern crate dotenv;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use handlers::get_poems_by_search;
use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod errors;
mod handlers;
pub mod models;
mod pagination;
pub mod schema;

use models::{NewPoem, Poems, ResPoems};
use schema::poems;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn establish_connection() -> Pool {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/guwen/selectall", web::get().to(handlers::get_poems))
            .route("/guwen/selectbyid", web::get().to(handlers::get_poem_by_id))
            .route(
                "/guwen/selectbywriter",
                web::get().to(handlers::get_poems_by_search),
            )
            .route(
                "/guwen/selectbydynasty",
                web::get().to(handlers::get_poems_by_search),
            )
            .route(
                "/guwen/selectbykeyword",
                web::get().to(handlers::get_poems_by_search),
            )
            .route(
                "/guwen/writer/selectall",
                web::get().to(handlers::get_writers),
            )
            .route(
                "/guwen/writer/selectbyname",
                web::get().to(handlers::get_writer_by_search),
            )
            .route(
                "/guwen/writer/selectbyid",
                web::get().to(handlers::get_writer_by_search),
            )
            .route(
                "/guwen/sentence/selectall",
                web::get().to(handlers::get_sentences),
            )
            .route(
                "/guwen/selectbysententce",
                web::get().to(handlers::get_poems_by_search),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
