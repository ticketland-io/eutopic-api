use actix_web::{web};
use super::{
  create_account,
  // get_account,
};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("")
    .route(web::post().to(create_account::exec))
  );
}
