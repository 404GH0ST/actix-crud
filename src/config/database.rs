use dotenv::dotenv;
use sqlx::postgres::PgPool;
use sqlx::Result;
use std::env;

pub async fn establish_connection() -> Result<PgPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
