mod html;
mod config;

use std::path::PathBuf;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use crate::html::HtmlBuilder;
use crate::config::SlapConfig;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(respond_static_site))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn respond_static_site() -> impl Responder {
    let path = PathBuf::from("slap.cfg");
    let config = SlapConfig::from_file(path);

    let mut markup = HtmlBuilder::from_config(config).build();

    HttpResponse::Ok().body(markup)
}