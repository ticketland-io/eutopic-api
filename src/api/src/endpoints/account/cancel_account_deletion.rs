use actix_web::{web, HttpResponse};
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

  // If delete_request_at is already None, return 400
  if account.delete_request_at.is_none() {
    return Ok(HttpResponse::BadRequest().finish())
  }

  postgres.update_delete_request_at(uid.clone(), None).await?;

  return Ok(HttpResponse::Ok().finish())
}
