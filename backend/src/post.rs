use actix_web::{post, web::ServiceConfig, HttpResponse, Responder};

pub fn post_services(cfg: &mut ServiceConfig) {
    cfg.service(echo);
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
