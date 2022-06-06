use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;
use sha2::Sha256;

pub type DB = Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> DB {
    let url = std::env::var("DATABASE_URL").expect("Could not find DATABASE_URL in .env");
    let manager = ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Could not build PostgreSQL connection pool.")
}

// DB actions
pub mod actions {
    use super::*;
    use crate::models::*;
    use diesel::prelude::*;

    pub mod users {
        use super::*;
        use crate::schema::users::dsl::*;

        // Immutable users actions
        pub mod immut {
            use super::*;

            pub fn get_users(conn: &PgConnection, name: &str, amount: i64) -> Vec<User> {
                let res = users
                    .filter(username.like(name))
                    .limit(amount)
                    .load::<User>(conn)
                    .expect("Error loading user");

                res
            }
        }

        // Mutable users actions
        pub mod muta {
            use super::*;

            pub fn create_user(conn: &PgConnection, req_body: &NewUser) -> Option<User> {
                match diesel::insert_into(users)
                    .values(req_body)
                    .get_result::<User>(conn)
                {
                    Ok(user) => Some(user),
                    Err(_) => None,
                }
            }
        }
    }
}
