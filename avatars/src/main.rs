#[macro_use]
extern crate lazy_static;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPool;

mod data;
mod imagekit;
mod models;

#[get("/avatar/{user_id}")]
async fn avatar(path: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
  match data::fetch_avatar(&pool, &path).await {
    Ok(avatar) => HttpResponse::TemporaryRedirect()
      .append_header(
        imagekit::ImageKitUrl::from(avatar.image_url)
          .height(300)
          .width(300)
          .redirect(),
      )
      .finish(),
    Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
    Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
  }
}

#[get("/healthz")]
async fn health(pool: web::Data<PgPool>) -> impl Responder {
  match data::health_check(&pool).await {
    Ok(_) => HttpResponse::Ok().body("Ok"),
    Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
  }
}

#[actix_web::main]
async fn main() -> Result<(), failure::Error> {
  dotenv().ok();

  let pool = web::Data::new(data::create_pool().await?);

  HttpServer::new(move || {
    App::new()
      .app_data(pool.clone())
      .service(health)
      .service(avatar)
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await?;

  Ok(())
}
