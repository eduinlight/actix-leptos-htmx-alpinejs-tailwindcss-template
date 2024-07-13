use std::sync::Mutex;

use actix_web::{http::header::ContentType, web::Data, HttpRequest, HttpResponse};
use leptos::*;

use crate::components::layouts::RootLayout;
use crate::{
  settings::{Environment, Settings},
  utils::http::headers::is_htmx_request,
};

pub fn render_root_layout<F, IV>(
  title: String,
  req: &HttpRequest,
  data: &Data<Mutex<Settings>>,
  render_page: F,
) -> HttpResponse
where
  F: Fn() -> IV + 'static,
  IV: IntoView,
{
  let body: String;
  let settings = data.lock().unwrap();
  let live_reload = settings.environment == Environment::DEVELOPMENT;
  let server_version = settings.server_version.clone();
  if is_htmx_request(req) {
    body = leptos::ssr::render_to_string(render_page).to_string();
  } else {
    body = format!(
      "<!DOCTYPE html>{}",
      leptos::ssr::render_to_string(move || {
        view! {
          <RootLayout
            title
            live_reload
            server_version
          >
            {render_page()}
          </RootLayout>
        }
      })
      .to_string()
    );
  }
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(body)
}
