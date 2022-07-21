use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use std::{env, panic, process};
use eutopic_api::{
  utils::store::Store,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    // invoke the default handler and exit the process
    orig_hook(panic_info);
    process::exit(1);
  }));

  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

  let store = web::Data::new(Store::new().await);
  let port = store.config.port;

  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  HttpServer::new(move || {
    App::new()
      .app_data(store.clone())
      .wrap(Cors::permissive())
      .wrap(middleware::Logger::default())
      .route("/", web::get().to(|| HttpResponse::Ok()))
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}
