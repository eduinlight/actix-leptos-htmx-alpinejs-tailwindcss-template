use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
  pub port: i16,
  pub live_reload: bool,
}

impl Settings {
  pub fn from_env() -> Self {
    Self {
      port: env::var("FRONT_PORT").unwrap().parse::<i16>().unwrap(),
      live_reload: env::var("FRONT_LIVE_RELOAD")
        .unwrap()
        .parse::<bool>()
        .unwrap(),
    }
  }
}
