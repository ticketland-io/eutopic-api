use std::sync::Arc;
use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use data::{
  helpers::{send_read},
  models::account::Account,
  repositories::account::{
    read_account,
    upsert_account
  },
};
use crate::{
  utils::store::Store,
  services::{
    data::{exec_basic_db_write_endpoint},
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
  let (query, db_query_params) = read_account(auth.user.local_id.clone());

  let account_result = send_read(Arc::clone(&store.neo4j), query, db_query_params)
  .await
  .map(|db_result| {
    TryInto::<Account>::try_into(db_result)
  });

  // if account exist then simply return the account
  if let Ok(account) = account_result {
    if let Ok(account) = account {
      return HttpResponse::Ok().body(serde_json::to_string(&account).expect("cannot serialize account"))
    }
  }
  
  // else create and store the encrypted mnemonic
  exec_basic_db_write_endpoint(
    store,
    Box::new(move || {
      upsert_account(
        auth.user.local_id.clone(),
        body.mnemonic.clone(),
      )
    })
  ).await;

  // Push message to Rabbitmq
  
  HttpResponse::Created().finish()
}
