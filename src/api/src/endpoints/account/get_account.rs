use std::sync::Arc;
use actix_web::{web, HttpResponse};
use data::{
  helpers::{send_read},
  models::account::Account,
  repositories::account::{
    read_account,
  },
};
use crate::{
  utils::store::Store,
  middlewares::auth::AuthData,
};

pub async fn exec(
  store: web::Data<Store>,
  auth: AuthData,
) -> HttpResponse {
  let (query, db_query_params) = read_account(auth.user.local_id);

  let account = send_read(Arc::clone(&store.neo4j), query, db_query_params)
    .await
    .map(|db_result| {
      TryInto::<Account>::try_into(db_result).unwrap()
    });

  if let Ok(_) = account {
    return HttpResponse::NotFound().finish()
  }
  
  HttpResponse::Ok().json(account.unwrap())
}
