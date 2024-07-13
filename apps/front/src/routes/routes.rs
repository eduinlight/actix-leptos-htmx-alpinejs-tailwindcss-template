use actix_web::web;

use super::home_router::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(home_router);
}
