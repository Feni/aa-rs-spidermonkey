#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate hyper;

#[macro_use]
pub mod schema;
pub mod models;
pub mod services;


use crate::services::resolve;
use crate::services::establish_connection;
use hyper::{Body, Request, Response, Server};
use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;

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



fn hello_world(req: Request<Body>) -> Response<Body> {
    println!("Request {:?}", req);
    println!("uri {:?}", req.uri());

    let host = req.headers().get("host").unwrap().to_str().unwrap().to_string();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    resolve(method, host, path);

    Response::new(Body::from(PHRASE))

}

fn main() {
    // pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 9080).into();
    let connection = establish_connection();
    
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr)
        // .serve(|| {
        //     // This is the `Service` that will handle the connection.
        //     // `service_fn_ok` is a helper to convert a function that
        //     // returns a Response into a `Service`.
        //     // service_fn_ok(move |_: Request<Body>| {
        //     //     Response::new(Body::from("Hello Arevel!"))
        //     // })
        // })
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}