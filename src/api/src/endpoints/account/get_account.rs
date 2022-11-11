use eyre::Result;
use actix_web::{web, HttpResponse};
use ticketland_core::error::Error;
use api_helpers::{
  middleware::auth::AuthData,
};
use crate::{
  utils::store::Store,
};

pub async fn exec(
  store: web::Data<Store>,
  auth: AuthData,
) -> Result<HttpResponse, Error> {
  let mut postgres = store.postgres.lock().unwrap();
  let account = postgres.read_account_by_id(auth.user.local_id).await?;
  
  Ok(HttpResponse::Ok().json(account))
}
