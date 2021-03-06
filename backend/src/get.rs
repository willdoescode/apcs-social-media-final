use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::db::DB;
use crate::{db, models::Post};

pub fn get_services(cfg: &mut ServiceConfig) {
    cfg.service(echo);
    cfg.service(get_user_by_username);
    cfg.service(get_posts_by_user);
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/get_user/{username}")]
async fn get_user_by_username(db: web::Data<DB>, username: web::Path<String>) -> impl Responder {
    let db_conn = db.get().expect("Error getting db conn");
    let maybe_user = db::actions::users::immut::get_users(&db_conn, &username, 1);

    HttpResponse::Ok().json(maybe_user.first())
}

#[get("/get_posts/{username}")]
async fn get_posts_by_user(db: web::Data<DB>, username: web::Path<String>) -> impl Responder {
    let db_conn = db.get().expect("Error getting db conn");

    let users = db::actions::users::immut::get_users(&db_conn, &username, 1);

    let user = users.first();
    let user = match user {
        Some(user) => user,
        None => return HttpResponse::Ok().json(None as Option<Vec<Post>>),
    };

    let posts = db::actions::posts::immut::get_posts_from_user(&db_conn, user);

    HttpResponse::Ok().json(posts)
}
