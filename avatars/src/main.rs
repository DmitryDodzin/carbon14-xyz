use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Pool;
use dotenv::dotenv;

mod pg_pool;

#[get("/healthz")]
async fn health(pool: web::Data<Pool>) -> impl Responder {
  match pg_pool::health_check(&pool).await {
    Ok(_) => HttpResponse::Ok().body("Ok"),
    Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
  }
}

#[actix_web::main]
async fn main() -> Result<(), failure::Error> {
  dotenv().ok();

  let pool = web::Data::new(pg_pool::create().await?);

  HttpServer::new(move || App::new().app_data(pool.clone()).service(health))
    .bind("0.0.0.0:3000")?
    .run()
    .await?;

  Ok(())
}
