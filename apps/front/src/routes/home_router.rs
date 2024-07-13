use std::sync::Mutex;

use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse};
use leptos::*;

use crate::components::layouts::RootLayout;
use crate::components::pages::Home;
use crate::settings::{Environment, Settings};
use crate::utils::http::headers::is_htmx_request;

#[get("/")]
pub async fn home_router(req: HttpRequest, data: Data<Mutex<Settings>>) -> HttpResponse {
  let body: String;
  if is_htmx_request(&req) {
    body = leptos::ssr::render_to_string(|| {
      view! {
        <Home />
      }
    })
    .to_string();
  } else {
    body = format!(
      "<!DOCTYPE html>{}",
      leptos::ssr::render_to_string(move || {
        let settings = data.lock().unwrap();
        view! {
          <RootLayout
            title="Todos".to_string()
            live_reload=settings.environment == Environment::DEVELOPMENT>
            <Home />
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
