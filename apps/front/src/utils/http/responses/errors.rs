use actix_web::{
  http::{header::ContentType, StatusCode},
  HttpResponse,
};
use tera::Error;

pub fn http_internal_server_error(error: Error) -> HttpResponse {
  println!("\n{:?}\n", error);
  HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
    .content_type(ContentType::html())
    .body("Server Error")
}
