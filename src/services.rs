
use diesel::query_builder::functions::sql_query;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::sql_types::Text;

use diesel::result::Error as err;

use crate::schema;
use crate::models::*;
use crate::js::exec_js;

use hyper::header;
use hyper::{Body, Request, Response, Server};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn resolve(q_method: String, q_host: String, q_path: String) -> Option<View> {
    // use schema::apps::dsl::*;
    // use schema::routes::dsl::*;
    // use schema::views::dsl::*;
    use schema::*;

    println!("Method: {} host {} path {}", q_method, q_host, q_path);
    let host_lower = q_host.to_lowercase();

    let pg_conn = establish_connection();

    let app_filter = apps::table.filter(apps::domain.eq(host_lower)).first::<App>(&pg_conn).unwrap();
    let mut results: Vec<(View, Route)> = View::belonging_to(&app_filter).inner_join(
        routes::table.on(
            views::id.eq(routes::view_id).and(
                routes::pattern.eq(q_path)
            )
        )
    ).limit(1)
    .load(&pg_conn)
    .expect("Error loading apps");

    // println!("PG result {:?}", results);
    
    if results.len() > 0 {
        let result = results.pop().unwrap();
        return Some(result.0);
    }

    return None;
}

pub fn dispatch(view: View) -> Response<Body> {
    if view.mime_type == "text/html" {
        let mut response = Response::new(Body::from(view.content.unwrap()));
        response.headers_mut().insert(header::CONTENT_TYPE, "text/html; charset=UTF-8".parse().unwrap());
        return response;
    } else if view.mime_type == "application/javascript" {
        return exec_js(view);
    } else {
        // Should not happen
        let mut response = Response::new(Body::from("AppAssembly Server Error"));
        return response;
    }
}

