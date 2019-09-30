#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate hyper;

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
use hyper::{Request, Response};
use hyper::server::Server;
use hyper::Body;
// use http::{Request, Response, Error};
// use hyper::rt::{self, Future};
use hyper::rt;
use hyper::service::{ Service, service_fn_ok };
use hyper::error::Error;
use hyper::StatusCode;


use diesel::pg::PgConnection;

// use hyper::server::{Request, Response, Service};

// fn landing(_req: HttpRequest) -> actix_web::Result<NamedFile> {
//     // let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     return Ok(NamedFile::open("/var/www/arevelcom/templates/landing.html")?)
// }

// fn arevel(_req: HttpRequest) -> actix_web::Result<NamedFile> {
//     return Ok(NamedFile::open("/var/www/arevelcom/templates/index.html")?)
// }

// fn health() -> impl Responder {
//     return "OK"
// }

// pub fn main() {
//     HttpServer::new(|| {
//         App::new()
//         .route("/", web::get().to(landing))
//         .route("/arevel", web::get().to(arevel))
//         .route("/_info/health", web::get().to(health))
//         .service(fs::Files::new("/static", "/var/www/arevelcom/static/"))  // static/dist/static
//     })
//     .bind("0.0.0.0:9080")
//     .expect("Can not bind to port 9080")
//     .run()
//     .unwrap();
// }

const PHRASE: &str = "Hello, World!";

pub struct AasmService {
    // js: Arc<JSEngine>
    id: i32
}


// fn finalize(result: Result<Response, Error>) -> Future<Item = Response, Error = Error> {
//     match result {
//         Ok(response) => ok(response),
//         Err(error) => {
//             let response_body = "Error response";
//             ok(Response::new()
//                 .with_status(StatusCode::INTERNAL_SERVER_ERROR)
//                 // .with_header(ContentLength(response_body.len() as u64))
//                 .with_body(response_body))
//         }
//     }
// }

impl Service for AasmService {
    // type Request = Request;
    // type Response = Response;
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    // type ReqBody = ReqBody;
    // type ResBody = ResBody;
    type Future = Box<Future<Item = Response<Body>, Error = Self::Error>>;
    
    
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // (self.f)(req).into_future()

        println!("Request {:?}", req);
        println!("uri {:?}", req.uri());

        // let host = req.headers().get("host").unwrap().to_str().unwrap().to_string();
        // let method = req.method().to_string();
        // let path = req.uri().path().to_string();

        // let maybe_view = resolve(method, host, path);
        // let mut resp;
        // if let Some(view) = maybe_view {
        //     resp = dispatch(view);
        // } else {
        //     let mut response = Response::new(Body::from("Not Found"));
        //     //let status = resp.status_mut();
        //     *response.status_mut() = StatusCode::NOT_FOUND;
        //     resp = response;
        // }

        // return Future::ok(resp);

        // let response = Ok(Response<Body>::new().with_status(StatusCode::OK));
        // Box::new(finalize(response))
        let response = Ok(Response::new(Body::from("Hello World!")));
        Box::new(future::ok(response))


    }
}

fn main() {
    // pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 9080).into();
    let connection = establish_connection();
    let aasm_svc = AasmService { id: 1 };

    // let server = Server::bind(&addr)
    //     // .serve(|| {
    //     //     // This is the `Service` that will handle the connection.
    //     //     // `service_fn_ok` is a helper to convert a function that
    //     //     // returns a Response into a `Service`.
    //     //     // service_fn_ok(move |_: Request<Body>| {
    //     //     //     Response::new(Body::from("Hello Arevel!"))
    //     //     // })
    //     // })
    //     .serve(new_svc)
    //     // .map_err(|e| eprintln!("server error: {}", e));

    // println!("Listening on http://{}", addr);

    // rt::run(server);

    // let server = Server::bind(&addr).serve(
    //     || {
    //         service_fn_ok(move |_: Request<Body>| {
    //             Response::new(Body::from("Hello Arevel!"))
    //         })

    //     }
    //     // move || Ok(aasm_svc)
    // );
    // // server.run().unwrap();
    // rt::run(server);



    let server = Server::bind(&addr)
        // .serve(|| {
        //     // This is the `Service` that will handle the connection.
        //     // `service_fn_ok` is a helper to convert a function that
        //     // returns a Response into a `Service`.
        //     service_fn_ok(move |req: Request<Body>| {
        //         hello_world(req, connection)
        //     })
        // })
        .serve(|| { aasm_svc })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);

}