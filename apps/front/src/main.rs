extern crate dotenv;

use std::sync::Mutex;

use actix::{Actor, StreamHandler};
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenv::dotenv;
use env_logger::Env;
use settings::Settings;

mod components;
mod routes;
mod settings;
mod utils;

/// Define HTTP actor
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

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LiveReloadWS {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Text(_)) => ctx.text(self.server_version.clone()),
      _ => (),
    }
  }
}

async fn index(
  req: HttpRequest,
  stream: web::Payload,
  data: Data<Mutex<Settings>>,
) -> Result<HttpResponse, Error> {
  let settings = data.lock().unwrap();
  let resp = ws::start(LiveReloadWS::from_settings(&settings), &req, stream);
  println!("{:?}", resp);
  resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  env_logger::init_from_env(Env::default().default_filter_or("info"));

  let settings = Settings::from_env();
  let data = Data::new(Mutex::new(settings.clone()));

  println!("{}", settings.live_reload);

  HttpServer::new(move || {
    let app = App::new()
      .app_data(data.clone())
      .wrap(Logger::default())
      .route("/ws-live-reload", web::get().to(index))
      .service(
        fs::Files::new("/static", "./apps/front/static")
          .show_files_listing()
          .use_last_modified(true),
      )
      .service(web::scope("").configure(routes::configure));

    app
  })
  .bind(format!("0.0.0.0:{}", settings.port))?
  .run()
  .await
}
