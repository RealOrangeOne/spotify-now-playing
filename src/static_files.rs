use include_dir::Dir;

use actix_web::{HttpRequest, HttpResponse};

static PROJECT_DIR: Dir = include_dir!("static/build");

pub fn serve_static(req: HttpRequest) -> HttpResponse {
    return match PROJECT_DIR.get_file(req.match_info().query("filename")) {
        Some(f) => HttpResponse::Ok().body(f.contents()),
        None => HttpResponse::NotFound().finish(),
    };
}
