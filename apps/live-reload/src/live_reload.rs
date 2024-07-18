use actix::{Actor, StreamHandler};
use actix_web::{
  get,
  web::{self},
  Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use uuid::Uuid;

struct LiveReloadWS {
  id: String,
}

impl LiveReloadWS {
  fn new() -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
    }
  }
}

impl Actor for LiveReloadWS {
  type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LiveReloadWS {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Text(_)) => ctx.text(""),
      _ => (),
    }
  }
}

#[get("/ws")]
pub async fn live_reload(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let resp = ws::start(LiveReloadWS::new(), &req, stream);
  println!("{:?}", resp);
  resp
}

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(live_reload);
}
