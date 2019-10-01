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
use actix_web::{web, App, Responder, HttpServer, HttpRequest, HttpResponse};
use diesel::pg::PgConnection;

use std::sync::Arc;
use mozjs::rust::JSEngine;

use crate::services::{ DBPool, AasmData, AasmState };
use actix_web::http::StatusCode;
use actix_web::dev::Body;




fn serve(req: HttpRequest, data: AasmData) -> HttpResponse {
    println!("Data is {:?}", data.id);

    let host = req.headers().get("host").unwrap().to_str().unwrap().to_string();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let pg_conn = &data.db.get().unwrap();
    let maybe_view = resolve(&pg_conn, method, host, path);

    if let Some(view) = maybe_view {
        return dispatch(view, data.js.clone());
    } else {
        // let mut response = Response::new(Body::from("Not Found"));
        // let status = resp.status_mut();
        // *response.status_mut() = StatusCode::NOT_FOUND;
        // return "Not Found";
        return HttpResponse::with_body(StatusCode::NOT_FOUND, Body::from("Not Found"))

    }
}

fn main() {
    let connection_pool = establish_connection();
    let JSE: Arc<JSEngine> = JSEngine::init().unwrap();
    let state = AasmState { id: 1, db: connection_pool, js: JSE };

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