use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::Level;
use simple_logger;

mod utils;
use utils::get_port;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() {
    let port = get_port();
    simple_logger::init_with_level(Level::Info).unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run()
    .unwrap();
}
