use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
  pub port: i16,
}

impl Settings {
  pub fn from_env() -> Self {
    Self {
      port: env::var("LIVE_RELOAD_PORT")
        .unwrap()
        .parse::<i16>()
        .unwrap(),
    }
  }
}
