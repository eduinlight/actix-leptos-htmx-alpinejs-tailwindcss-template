#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
  PRODUCTION,
  DEVELOPMENT,
}

impl Environment {
  pub fn from_str(str: &str) -> Self {
    match str {
      "production" => Environment::PRODUCTION,
      "development" => Environment::DEVELOPMENT,
      _ => Environment::PRODUCTION,
    }
  }
}
