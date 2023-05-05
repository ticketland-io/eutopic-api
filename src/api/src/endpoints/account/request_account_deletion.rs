use actix_web::{web, HttpResponse};
use chrono::Utc;
use ticketland_core::error::Error;
use api_helpers::middleware::auth::AuthData;
use crate::utils::store::Store;

pub async fn exec(
  store: web::Data<Store>,
  auth: AuthData,
) -> Result<HttpResponse, Error> {
  let uid = auth.user.local_id.clone();
  let mut postgres = store.pg_pool.connection().await?;

  let Ok(account) = postgres.read_account_by_id(uid.clone()).await else {
    return Ok(HttpResponse::NotFound().finish())
  };

  // If delete_request_at is already set, return 400
  if account.delete_request_at.is_some() {
    return Ok(HttpResponse::BadRequest().finish())
  }

  postgres.update_delete_request_at(
    uid.clone(),
    Some(Utc::now().naive_utc()),
  ).await?;

  // Push delete_account_request message to Rabbitmq
  store.delete_account_request_queue.on_new_delete_request(uid.clone()).await?;

  return Ok(HttpResponse::Ok().finish())
}
