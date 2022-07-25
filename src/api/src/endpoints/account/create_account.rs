use std::sync::Arc;
use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use common_data::{
  helpers::{send_read},
  models::account::Account,
  repositories::account::{
    read_account,
    upsert_account
  },
};
use api_helpers::{
  middleware::auth::AuthData,
  services::{
    data::{exec_basic_db_write_endpoint},
  },
};
use crate::{
  utils::store::Store,
};

#[derive(Deserialize)]
pub struct Body {
  mnemonic: String,
  address: String,
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
  
  let address = body.address.clone();
  // else create and store the encrypted mnemonic
  exec_basic_db_write_endpoint(
    Arc::clone(&store.neo4j),
    Box::new(move || {
      upsert_account(
        auth.user.local_id.clone(),
        body.mnemonic.clone(),
      )
    })
  ).await;

  // Push message to Rabbitmq
  store.new_user_queue.on_new_user(address).await;

  HttpResponse::Created().finish()
}
