use diesel::Queryable;

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub bio: Option<String>,
    pub is_admin: bool,
}
