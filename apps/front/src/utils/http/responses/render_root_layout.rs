use std::sync::Mutex;

use actix_web::{http::header::ContentType, web::Data, HttpRequest, HttpResponse};
use tera::Result;

use crate::components::layouts::{root_layout, RootLayoutProps};
use crate::{
  settings::{Environment, Settings},
  utils::http::headers::is_htmx_request,
};

use super::http_internal_server_error;

pub fn render_root_layout(
  title: String,
  req: &HttpRequest,
  data: &Data<Mutex<Settings>>,
  page: Result<String>,
) -> HttpResponse {
  let response: HttpResponse;
  let settings = data.lock().unwrap();
  let live_reload = settings.environment == Environment::DEVELOPMENT;
  let server_version = settings.server_version.clone();
  match page {
    Ok(body) => {
      if is_htmx_request(req) {
        response = HttpResponse::Ok()
          .content_type(ContentType::html())
          .body(body);
      } else {
        match root_layout(RootLayoutProps {
          live_reload,
          server_version,
          title,
          children: Some(body),
        }) {
          Ok(body) => {
            response = HttpResponse::Ok()
              .content_type(ContentType::html())
              .body(body);
          }
          _ => response = http_internal_server_error(None),
        }
      }
    }
    _ => response = http_internal_server_error(None),
  }
  response
}
