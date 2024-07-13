use std::sync::Mutex;

use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse};
use leptos::view;

use crate::components::pages::Home;
use crate::settings::Settings;
use crate::utils::http::responses::render_root_layout;

#[get("/")]
pub async fn home_router(req: HttpRequest, data: Data<Mutex<Settings>>) -> HttpResponse {
  render_root_layout("Todos".to_string(), &req, &data, || view! {<Home />})
}
