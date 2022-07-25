use std::sync::Arc;
use actix_web::{web, HttpResponse};
use common_data::{
  helpers::{send_read},
  models::account::Account,
  repositories::account::{
    read_account,
  },
};
use ticketland_core::error::Error;
use api_helpers::{
  middleware::auth::AuthData,
  services::http::internal_server_error,
};
use crate::{
  utils::store::Store,
};

pub async fn exec(
  store: web::Data<Store>,
  auth: AuthData,
) -> HttpResponse {
  let (query, db_query_params) = read_account(auth.user.local_id);

  send_read(Arc::clone(&store.neo4j), query, db_query_params)
    .await
    .map(|db_result| {
      TryInto::<Account>::try_into(db_result)
    })
    .map(|account| {
      if let Ok(account) = account {
        HttpResponse::Ok().json(account)
      } else {
        HttpResponse::NotFound().finish()
      }
    })
    .unwrap_or_else(|error: Error| internal_server_error(Some(error)))
}
