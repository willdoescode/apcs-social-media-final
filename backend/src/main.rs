mod get;
mod post;

use actix_web::{middleware::Logger, App, HttpServer};

fn init_logging() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    HttpServer::new(|| {
        let app = App::new()
            .wrap(Logger::default())
            .configure(get::get_services)
            .configure(post::post_services);

        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
