use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};


fn landing(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    // let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    return Ok(NamedFile::open("/var/www/arevelcom/templates/landing.html")?)
}

fn arevel(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    return Ok(NamedFile::open("/var/www/arevelcom/templates/index.html")?)
}

fn health() -> impl Responder {
    return "OK"
}

pub fn main() {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(landing))
        .route("/arevel", web::get().to(arevel))
        .route("/_info/health", web::get().to(health))
        .service(fs::Files::new("/static", "/var/www/arevelcom/static/"))  // static/dist/static
    })
    .bind("0.0.0.0:9080")
    .expect("Can not bind to port 9080")
    .run()
    .unwrap();
}
