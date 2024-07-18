use actix_files as fs;
use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod live_reload;
mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let settings = settings::Settings::from_env();

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .wrap(Compress::default())
      .service(
        fs::Files::new("/static", "./apps/live-reload/static")
          .show_files_listing()
          .use_last_modified(true),
      )
      .service(web::scope("").configure(live_reload::configure))
  })
  .bind(format!("0.0.0.0:{}", settings.port))?
  .run()
  .await
}
