use std::env;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Settings {
  pub port: i16,
  pub live_reload: bool,
  pub server_version: String,
}

impl Settings {
  pub fn from_env() -> Self {
    Self {
      port: env::var("FRONT_PORT").unwrap().parse::<i16>().unwrap(),
      live_reload: env::var("FRONT_LIVE_RELOAD")
        .unwrap()
        .parse::<bool>()
        .unwrap(),
      server_version: Uuid::new_v4().to_string(),
    }
  }
}
