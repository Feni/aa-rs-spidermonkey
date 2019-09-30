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
use actix_web::{web, App, Responder, HttpServer, HttpRequest};
use diesel::pg::PgConnection;

use std::sync::Arc;
use mozjs::rust::JSEngine;

use crate::services::{ DBPool, AasmData, AasmState };




fn serve(req: HttpRequest, data: AasmData) -> impl Responder {
    println!("Data is {:?}", data.id);
    return "Hello World"
}

fn main() {
    // pretty_env_logger::init();
    // let addr = ([127, 0, 0, 1], 9080).into();
    let connection = establish_connection();

    let JSE: Arc<JSEngine> = JSEngine::init().unwrap();
    let state = AasmState { id: 1, conn: connection, js: JSE };

    HttpServer::new(move || App::new()
    .data(
        state.clone()
    )
    .service(
        web::resource("*").to(serve))
    )
    .bind("127.0.0.1:9080").unwrap()
    .run();
}