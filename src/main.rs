use std::{env, io};

use actix_web::{get, App, HttpResponse, HttpServer};
use argh::FromArgs;
use log::info;

const INDEX_HTML: &'static str = include_str!("../web/dist/index.html");

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_IP: &'static str = "127.0.0.1";

/// A simple web interface to manage your server.
#[derive(FromArgs)]
struct CliArgs {
    /// what port to serve on
    #[argh(option, short = 'p', default = "DEFAULT_PORT")]
    pub port: u16,
    /// what ip to serve on
    #[argh(option, short = 'i', default = "DEFAULT_IP.to_string()")]
    pub ip: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let args: CliArgs = argh::from_env();

    info!("Starting server on http://{}:{}/", &args.ip, &args.port);

    HttpServer::new(|| App::new().service(index))
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
