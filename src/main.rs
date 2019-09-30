#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate mozjs;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod schema;
pub mod models;
pub mod services;
pub mod js;
pub mod timing;


use crate::services::{resolve, dispatch, establish_connection};
use futures::future::{ok, Future};
use actix_web::{web, App, Responder, HttpServer};

use diesel::pg::PgConnection;


pub struct AasmService {
    // js: Arc<JSEngine>
    id: i32
}

fn hello() -> impl Responder {
    return "Hello World"
}

fn main() {
    // pretty_env_logger::init();
    // let addr = ([127, 0, 0, 1], 9080).into();
    let connection = establish_connection();
    let aasm_svc = AasmService { id: 1 };

    HttpServer::new(|| App::new().service(
        web::resource("*").to(hello))
    )
    .bind("127.0.0.1:9080").unwrap()
    .run();
}