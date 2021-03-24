use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use crate::models::Avatar;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect_with(PgConnectOptions::new())
    .await?;

  Ok(pool)
}

pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
  let _: (i64,) = sqlx::query_as("SELECT 1 + 1").fetch_one(pool).await?;

  Ok(())
}

pub async fn fetch_avatar(pool: &PgPool, user_id: &str) -> Result<Avatar, sqlx::Error> {
  let avatar = sqlx::query_as(
    "
  SELECT 
    user_id,
    platform,
    image_url,
    image_height,
    image_width,
    image_mimetype
  FROM avatars_api.avatar_images
    WHERE user_id = $1
  LIMIT 1;
  ",
  )
  .bind(user_id)
  .fetch_one(pool)
  .await?;

  Ok(avatar)
}
