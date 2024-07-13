use std::env;

use uuid::Uuid;

use super::environment::*;

#[derive(Debug, Clone)]
pub struct Settings {
  pub port: i16,
  pub environment: Environment,
  pub server_version: String,
}

impl Settings {
  pub fn from_env() -> Self {
    Self {
      port: env::var("FRONT_PORT").unwrap().parse::<i16>().unwrap(),
      environment: Environment::from_str(&env::var("RUST_ENV").unwrap()),
      server_version: Uuid::new_v4().to_string(),
    }
  }
}
