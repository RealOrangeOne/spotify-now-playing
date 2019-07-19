use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};
use spotify_oauth::{SpotifyAuth, SpotifyCallback, SpotifyScope};
use std::env;
use std::str::FromStr;

pub fn spotify_auth() -> Scope {
    return web::scope("/auth")
        .route("/init", web::get().to(authorize_user))
        .route("/callback", web::get().to(callback));
}

fn authorize_user() -> impl Responder {
    let auth = SpotifyAuth {
        scope: vec![SpotifyScope::UserReadPlaybackState],
        redirect_uri: "http://localhost:5000/auth/callback"
            .parse()
            .expect("Failed to parse redirect uri"),
        client_id: env::var("SPOTIFY_CLIENT_ID").expect("Missing client ID"),
        show_dialog: true,
        ..Default::default()
    };
    return HttpResponse::TemporaryRedirect()
        .set_header(
            "Location",
            auth.authorize_url().expect("Failed to build auth url"),
        )
        .finish();
}

fn callback(req: HttpRequest) -> impl Responder {
    let abs_uri = "http://localhost:5000".to_owned() + req.uri().to_string().as_str();
    let spotify_callback = SpotifyCallback::from_str(&abs_uri).expect("Invalid callback request");
    println!("{:?}", spotify_callback);
    let token = spotify_callback
        .convert_into_token()
        .expect("Failed to convert into token");
    println!("{:?}", token);
    return HttpResponse::Ok().finish();
}
