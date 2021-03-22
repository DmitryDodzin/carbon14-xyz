use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

pub async fn create() -> Result<Pool, failure::Error> {
  let cfg = Config {
    manager: Some(ManagerConfig {
      recycling_method: RecyclingMethod::Fast,
    }),
    ..envy::prefixed("PG_").from_env::<Config>().unwrap()
  };

  let pool = cfg.create_pool(NoTls)?;

  Ok(pool)
}

pub async fn health_check(pool: &Pool) -> Result<(), failure::Error> {
  let client = pool.get().await?;
  let stmt = client.prepare("SELECT 1 + 1").await?;
  client.query(&stmt, &[]).await?;

  Ok(())
}
