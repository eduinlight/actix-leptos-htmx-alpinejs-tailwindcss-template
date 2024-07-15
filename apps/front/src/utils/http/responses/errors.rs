use actix_web::{
  http::{header::ContentType, StatusCode},
  HttpResponse,
};

pub fn http_internal_server_error(error: Option<String>) -> HttpResponse {
  HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
    .content_type(ContentType::html())
    .body(error.unwrap_or("Server Error".to_string()))
}
