use regex::Regex;

lazy_static! {
  static ref UNWRAP_URL: Regex = Regex::new(r"^(https://[\w\d.-]+/[\w\d.-]+)(.*)$").unwrap();
}

#[derive(Default)]
struct Transformation {
  height: Option<u32>,
  width: Option<u32>,
}

impl Transformation {
  fn is_some(&self) -> bool {
    self.height.is_some() || self.width.is_some()
  }

  fn tokens(&self) -> Vec<String> {
    let mut tokens = Vec::new();

    if let Some(height) = self.height {
      tokens.push(format!("h-{}", height));
    }
    if let Some(width) = self.width {
      tokens.push(format!("w-{}", width));
    }

    tokens
  }
}

pub struct ImageKitUrl {
  endpoint_url: String,
  image_path: String,
  transformations: Transformation,
}

impl From<String> for ImageKitUrl {
  fn from(url: String) -> Self {
    let value = UNWRAP_URL.captures_iter(&url).next().unwrap();

    ImageKitUrl {
      endpoint_url: value[1].to_string(),
      image_path: value[2].to_string(),
      transformations: Transformation::default(),
    }
  }
}

impl ImageKitUrl {
  pub fn height(&mut self, height: u32) -> &mut Self {
    self.transformations.height = Some(height);
    self
  }

  pub fn width(&mut self, width: u32) -> &mut Self {
    self.transformations.width = Some(width);
    self
  }

  pub fn redirect(&self) -> (&'static str, String) {
    let target = if self.transformations.is_some() {
      format!(
        "{}/tr:{}{}",
        self.endpoint_url,
        self.transformations.tokens().join(","),
        self.image_path,
      )
    } else {
      format!("{}{}", self.endpoint_url, self.image_path)
    };
    ("location", target)
  }
}
