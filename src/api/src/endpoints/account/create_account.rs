use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use data::{
  repositories::account::upsert_account,
};
use crate::{
  utils::store::Store,
  services::{
    data::exec_basic_db_write_endpoint,
  },
  middlewares::auth::AuthData,
};

#[derive(Deserialize)]
pub struct Body {
  mnemonic: String,
}

pub async fn exec(
  store: web::Data<Store>,
  body: web::Json<Body>,
  auth: AuthData,
) -> HttpResponse {
  exec_basic_db_write_endpoint(
    store,
    Box::new(move || {
      upsert_account(
        auth.user.local_id.clone(),
        body.mnemonic.clone(),
      )
    })
  ).await;

  HttpResponse::Created().finish()
}
