use actix_web::HttpRequest;

pub fn get_referer_header(req: &HttpRequest) -> String {
  match req.headers().get("Referer") {
    Some(header) => match header.to_str() {
      Ok(value) => value.to_string(),
      _ => "".to_string(),
    },
    _ => "".to_string(),
  }
}
