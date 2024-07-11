use actix_web::web;

use super::home::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(home_router);
}
