use actix_web::{web};
use super::{
  create_account,
  get_account,
  request_account_deletion,
};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("")
    .route(web::post().to(create_account::exec))
    .route(web::get().to(get_account::exec))
    .route(web::delete().to(request_account_deletion::exec))
  );
}
