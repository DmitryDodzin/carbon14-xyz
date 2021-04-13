use twitch_api::*;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
  let auth: Auth = get_auth("", "").await?;

  println!("{:?}", &auth);

  Ok(())
}
