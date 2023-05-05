use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use ticketland_core::error::Error;
use api_helpers::middleware::auth::AuthData;
use crate::utils::store::Store;

#[derive(Deserialize)]
pub struct QueryString {
  delete_request: bool,
}

pub async fn exec(
  store: web::Data<Store>,
  qs: web::Query<QueryString>,
  auth: AuthData,
) -> Result<HttpResponse, Error> {
  let uid = auth.user.local_id.clone();
  let delete_request = qs.delete_request;
  let mut postgres = store.pg_pool.connection().await?;

  if let Ok(account) = postgres.read_account_by_id(uid.clone()).await {

    // If delete_request_at is present in account, a new one can't be requested
    if account.delete_request_at.is_some() && delete_request {
      return Ok(HttpResponse::BadRequest().finish())
    }

    postgres.update_delete_request_at(
      uid.clone(),
      delete_request.then(|| Utc::now().naive_utc())
    ).await?;

    // Push message to Rabbitmq if delete_request qs param is `true`
    if delete_request {
      store.delete_account_request_queue.on_new_delete_request(uid.clone()).await?;
    }

    return Ok(HttpResponse::Ok().finish())
  };

  // if account doesn't exist return error
  Ok(HttpResponse::InternalServerError().finish())
}
