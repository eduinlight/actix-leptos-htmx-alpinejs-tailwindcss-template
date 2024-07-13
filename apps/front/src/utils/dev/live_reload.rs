use std::sync::Mutex;

use actix::{Actor, StreamHandler};
use actix_web::{
  get,
  web::{self, Data},
  Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use crate::{settings::Settings, utils::http::headers::get_referer_header};

struct LiveReloadWS {
  server_version: String,
}

impl LiveReloadWS {
  pub fn from_settings(settings: &Settings) -> Self {
    Self {
      server_version: settings.server_version.clone(),
    }
  }
}

impl Actor for LiveReloadWS {
  type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LiveReloadWS {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Text(_)) => ctx.text(self.server_version.clone()),
      _ => (),
    }
  }
}

#[get("/ws")]
pub async fn live_reload(
  req: HttpRequest,
  stream: web::Payload,
  data: Data<Mutex<Settings>>,
) -> Result<HttpResponse, Error> {
  let settings = data.lock().unwrap();
  let resp = ws::start(LiveReloadWS::from_settings(&settings), &req, stream);
  println!("{:?}", resp);
  resp
}

#[get("/refresh")]
pub async fn refresh() -> HttpResponse {
  HttpResponse::Ok()
    .insert_header(("HX-Refresh", "true"))
    .finish()
}

pub fn configure_live_reload(cfg: &mut web::ServiceConfig) {
  cfg.service(live_reload);
  cfg.service(refresh);
}
