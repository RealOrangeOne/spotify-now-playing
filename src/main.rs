#[macro_use]
extern crate include_dir;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::Level;
use simple_logger;

mod utils;
use utils::get_port;
mod static_files;
use static_files::serve_static;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

fn index() -> impl Responder {
    let template = IndexTemplate { title: "Test" };
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap());
}

fn main() {
    let port = get_port();
    simple_logger::init_with_level(Level::Info).unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/static/{filename:.*}", web::get().to(serve_static))
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run()
    .unwrap();
}
