use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::db;
use crate::db::DB;

pub fn get_services(cfg: &mut ServiceConfig) {
    cfg.service(echo);
    cfg.service(get_user_by_username);
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct Username {
    username: String,
}

#[get("/get_user")]
async fn get_user_by_username(db: web::Data<DB>, username: web::Json<Username>) -> impl Responder {
    let db_conn = db.get().expect("Error getting db conn");
    let maybe_user = db::actions::users::immut::get_users(&username.username, 1, &db_conn);

    HttpResponse::Ok().json(maybe_user.first())
}
