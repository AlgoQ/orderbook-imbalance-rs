use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::env;

pub async fn create_pool_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    Ok(pool)
}