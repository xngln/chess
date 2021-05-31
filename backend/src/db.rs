use sqlx::postgres::PgPool;
use std::env;

pub async fn create_connection_pool() -> PgPool {
    let db_url_key = "DATABASE_URL";
    let db_url = env::var(db_url_key).expect("Couldn't get DB URL");
    PgPool::connect(&db_url).await.expect("Couldn't create db connection pool")
}