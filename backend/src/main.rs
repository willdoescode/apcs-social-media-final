#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde;

mod db;
mod get;
mod models;
mod post;
mod schema;

use dotenv::dotenv;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let host = std::env::var("HOST");
    let port = std::env::var("port");

    let host = match host {
        Ok(h) => h,
        Err(_) => "127.0.0.1".to_string(),
    };

    let port = match port {
        Ok(p) => p.parse::<u16>().expect("Could not parse port from .env"),
        Err(_) => 8080,
    };

    let pool = db::pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let app = App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(get::get_services)
            .configure(post::post_services);

        app
    })
    .bind((host, port))?
    .run()
    .await
}
