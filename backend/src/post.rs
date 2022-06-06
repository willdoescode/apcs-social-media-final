use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::db;
use crate::db::DB;
use crate::models::NewUser;

use sha2::{Digest, Sha256};

pub fn post_services(cfg: &mut ServiceConfig) {
    cfg.service(echo);
    cfg.service(create_user);
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct CreateUserInfo {
    username: String,
    bio: Option<String>,
    password: String,
}

#[post("/create_user")]
async fn create_user(
    db: web::Data<DB>,
    create_user_info: web::Json<CreateUserInfo>,
) -> impl Responder {
    let db_conn = db.get().expect("Error getting db conn");

    let unhashed = &create_user_info.password;

    let mut hasher = Sha256::new();
    hasher.update(unhashed);

    let hashed_password: String = format!("{:X}", hasher.finalize());

    let maybe_user = db::actions::users::muta::create_user(
        &db_conn,
        &NewUser {
            username: &create_user_info.username,
            bio: create_user_info.bio.as_deref(),
            password: &hashed_password,
        },
    );

    HttpResponse::Ok().json(maybe_user)
}

#[post("/create_post")]
async fn create_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
