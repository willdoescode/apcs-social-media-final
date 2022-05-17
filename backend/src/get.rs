use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

pub fn get_services(cfg: &mut ServiceConfig) {
    cfg.service(echo);
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
