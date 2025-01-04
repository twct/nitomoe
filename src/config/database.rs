use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_db_connection(database_url: &str) -> PgPool {
    init_db_pool(database_url).expect("Failed to create database pool")
}

fn init_db_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
