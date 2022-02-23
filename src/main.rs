mod html;
mod config;
mod card;

use std::net::Ipv4Addr;
use std::path::PathBuf;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_files::NamedFile;
use crate::card::Card;
use crate::html::HtmlBuilder;
use crate::config::Config;

const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 8080;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(respond_static_site))
            .route("/{filename:.*}", web::get().to(serve_file))
    })
        .bind((IP, PORT))?
        .run()
        .await
}

async fn serve_file(req: HttpRequest) -> Result<NamedFile, std::io::Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn respond_static_site() -> impl Responder {
    let path = PathBuf::from("slap.cfg");
    let mut config = Config::from_file(path);

    config.title = "This is a title.".to_string();
    config.n_rows = 3;
    config.n_cols = 3;

    let card1 = Card::new()
        .title("InfluxDB".to_string())
        .link("http://192.168.178.102:8086".to_string())
        .image_link(format!("http://{}:{}/{}", IP, PORT, "rust.jpg"))
        .description("Energy monitoring".to_string());

    let card2 = Card::new()
        .title("Github1".to_string())
        .link("http://github.com/twowaysix".to_string())
        .image_link(format!("http://{}:{}/{}", IP, PORT, "rust.jpg"))
        .description("TwoWaySix Github".to_string());

    let card3 = Card::new()
        .title("Github2".to_string())
        .link("http://github.com/twowaysix".to_string())
        .image_link(format!("http://{}:{}/{}", IP, PORT, "rust.jpg"))
        .description("TwoWaySix Github".to_string());

    let card4 = Card::new()
        .title("Github3".to_string())
        .link("http://github.com/twowaysix".to_string())
        .image_link(format!("http://{}:{}/{}", IP, PORT, "rust.jpg"))
        .description("TwoWaySix Github".to_string());

    let card5 = Card::new()
        .title("Github4".to_string())
        .link("http://github.com/twowaysix".to_string())
        .image_link(format!("http://{}:{}/{}", IP, PORT, "rust.jpg"))
        .description("TwoWaySix Github".to_string());

    let card6 = Card::new()
        .title("Github5".to_string())
        .link("http://github.com/twowaysix".to_string())
        .image_link(format!("http://{}:{}/{}", IP, PORT, "rust.jpg"))
        .description("TwoWaySix Github".to_string());

    let card7 = Card::new()
        .title("Github6".to_string())
        .link("http://github.com/twowaysix".to_string())
        .image_link(format!("http://{}:{}/{}", IP, PORT, "rust.jpg"))
        .description("TwoWaySix Github".to_string());

    config.cards.push(card1);
    config.cards.push(card2);
    config.cards.push(card3);
    config.cards.push(card4);
    config.cards.push(card5);
    config.cards.push(card6);
    config.cards.push(card7);

    let mut markup = HtmlBuilder::from_config(config).build();

    HttpResponse::Ok().body(markup)
}