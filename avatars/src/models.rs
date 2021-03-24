#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Avatar {
  pub user_id: String,
  pub platform: String,
  pub image_url: String,
  pub image_height: i64,
  pub image_width: i64,
  pub image_mimetype: String,
}
