use actix_web::HttpRequest;

pub fn is_htmx_request(req: HttpRequest) -> bool {
  match req.headers().get("HX-Request") {
    Some(header) => match header.to_str() {
      Ok(value) => value == "true",
      _ => false,
    },
    _ => false,
  }
}
