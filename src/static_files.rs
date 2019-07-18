use include_dir::Dir;

use actix_web::{HttpRequest, HttpResponse, Responder};

static PROJECT_DIR: Dir = include_dir!("static/build");

pub fn serve_static(req: HttpRequest) -> impl Responder {
    return match PROJECT_DIR.get_file(req.match_info().query("filename")) {
        Some(f) => HttpResponse::Ok().body(f.contents()),
        None => HttpResponse::NotFound().finish(),
    };
}
