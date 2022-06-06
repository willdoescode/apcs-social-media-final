#[allow(unused_imports)]
use crate::schema::{posts, users};

use diesel::{Associations, Identifiable, Queryable};

#[derive(Queryable, Identifiable, Associations, Clone, Serialize, Deserialize)]
#[primary_key(username)]
pub struct User {
    pub username: String,
    pub bio: Option<String>,
    pub is_admin: bool,
    pub password: String,
}

#[derive(Identifiable, Queryable, Associations, Clone, Serialize, Deserialize)]
#[belongs_to(User, foreign_key = "author")]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub bio: Option<&'a str>,
    pub password: &'a str,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub author: &'a str,
}
