use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

pub type DB = Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> DB {
    let url = std::env::var("DATABASE_URL").expect("Could not find DATABASE_URL in .env");
    let manager = ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Could not build PostgreSQL connection pool.")
}
