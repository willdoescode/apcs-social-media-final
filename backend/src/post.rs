use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::models::{NewUser, User};
use crate::{db, models::Post};
use crate::{db::DB, models::NewPost};

use sha2::{Digest, Sha256};

pub fn post_services(cfg: &mut ServiceConfig) {
    cfg.service(echo);
    cfg.service(create_user);
    cfg.service(create_post);
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

    let users = db::actions::users::immut::get_users(&db_conn, &create_user_info.username, 1);
    if !users.is_empty() {
        return HttpResponse::Ok().json(None as Option<User>);
    }

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

#[derive(Deserialize)]
struct CreatePostInfo {
    username: String,
    password: String,
    title: String,
    body: String,
}

#[post("/create_post")]
async fn create_post(
    db: web::Data<DB>,
    login_user_info: web::Json<CreatePostInfo>,
) -> impl Responder {
    let db_conn = db.get().expect("Error getting db conn");

    let users = db::actions::users::immut::get_users(&db_conn, &login_user_info.username, 1);

    let user = users.first();
    let user = match user {
        Some(user) => user,
        None => return HttpResponse::Ok().json(None as Option<Post>),
    };

    let mut hasher = Sha256::new();
    hasher.update(&login_user_info.password);

    let hashed_password: String = format!("{:X}", hasher.finalize());

    if hashed_password != user.password {
        return HttpResponse::Ok().json(None as Option<Post>);
    }

    let new_post = db::actions::posts::muta::create_post(
        &db_conn,
        &NewPost {
            title: &login_user_info.title,
            body: &login_user_info.body,
            author: &user.username,
        },
    );

    HttpResponse::Ok().json(new_post)
}
