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

  let mnemonic = send_read(Arc::clone(&store.neo4j), query, db_query_params)
  .await
  .map(|db_result| {
    let account: Account = db_result.try_into().unwrap();

    return account.mnemonic;
  });

  // if account exist then simply return the encrypted mnemonic
  if let Ok(mnemonic) = mnemonic {
    return HttpResponse::Ok().body(mnemonic)
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

  HttpResponse::Created().finish()
}
