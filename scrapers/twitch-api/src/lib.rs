use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
  Bearer,
}

impl From<TokenType> for String {
  fn from(r#type: TokenType) -> Self {
    match r#type {
      TokenType::Bearer => "bearer".to_string(),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct Auth {
  pub token_type: TokenType,
  pub access_token: String,
  pub refresh_token: Option<String>,
  pub expires_in: u32,
}

pub async fn get_auth(client_id: &str, client_secret: &str) -> Result<Auth, reqwest::Error> {
  let client = reqwest::Client::new();

  Ok(
    client
      .post("https://id.twitch.tv/oauth2/token")
      .query(&[
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
      ])
      .send()
      .await?
      .json()
      .await?,
  )
}
