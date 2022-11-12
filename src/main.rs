use std::{env, io};

use actix_web::{get, web::{Data, Path}, App, HttpResponse, HttpServer};
use argh::FromArgs;
use log::info;

mod resources;
mod info;

const INDEX_HTML: &'static str = include_str!("../web/dist/index.html");

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_IP: &'static str = "127.0.0.1";
const DEFAULT_TICK_LENGTH: u64 = 500;
const DEFAULT_PASSWORD: &'static str = "CHANGE ME";

/// A simple web interface to manage your server.
#[derive(FromArgs, Clone, Debug)]
struct Args {
    /// what port to serve on
    #[argh(option, short = 'p', default = "DEFAULT_PORT")]
    pub port: u16,
    /// what ip to serve on
    #[argh(option, short = 'i', default = "DEFAULT_IP.to_string()")]
    pub ip: String,
    /// number of milliseconds to wait between system updates
    #[argh(option, short = 't', default = "DEFAULT_TICK_LENGTH")]
    pub tick_length: u64,
    /// password in the web interface
    #[argh(option, short = 'a', default ="DEFAULT_PASSWORD.to_string()")]
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    args: Args,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let args: Args = argh::from_env();

    info!("Starting server on http://{}:{}/", &args.ip, &args.port);

    let state = AppState { args: args.clone() };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(index)
            .service(auth)
            .service(resources::broadcast)
            .service(info::info)
    })
    .bind((args.ip, args.port))?
    .run()
    .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(INDEX_HTML)
}

/// returns "true" if good password or "false" if bad one
#[get("/auth/{password}")]
async fn auth(data: Data<AppState>, password: Path<String>) -> HttpResponse {
    if data.args.password == password.to_string() {
        HttpResponse::Ok().body("true")
    } else {
        HttpResponse::Ok().body("false")
    }
}