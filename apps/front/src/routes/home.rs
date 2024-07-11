use actix_web::http::header::ContentType;
use actix_web::{get, HttpRequest, HttpResponse};
use leptos::*;

use crate::components::layouts::RootLayout;
use crate::components::pages::Home;
use crate::utils::is_htmx_request;

#[get("/")]
pub async fn home_router(req: HttpRequest) -> HttpResponse {
  let body: String;
  if is_htmx_request(req.clone()) {
    body = leptos::ssr::render_to_string(|cx| {
      view! {cx,
        <Home />
      }
    });
  } else {
    body = format!(
      "<!DOCTYPE html>{}",
      leptos::ssr::render_to_string(|cx| {
        view! {cx,
          <RootLayout title="Todos".to_string()>
            <Home />
          </RootLayout>
        }
      })
    );
  }
  HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(body)
}
