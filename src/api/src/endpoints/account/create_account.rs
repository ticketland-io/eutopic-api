use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use ticketland_core::error::Error;
use ticketland_data::models::account::Account;
use api_helpers::{
  middleware::auth::AuthData,
};
use crate::{
  utils::store::Store,
};

#[derive(Deserialize)]
pub struct Body {
  mnemonic: String,
  pubkey: String,
}

pub async fn exec(
  store: web::Data<Store>,
  body: web::Json<Body>,
  auth: AuthData,
) -> Result<HttpResponse, Error> {
  let uid = auth.user.local_id.clone();

  let mut postgres = store.pg_pool.connection().await?;
  let Ok(account) = postgres.read_account_by_id(uid.clone()).await else {
    // create and store the encrypted mnemonic
    let pubkey = body.pubkey.clone();

    postgres.upsert_account(Account {
      uid,
      created_at: None,
      mnemonic: body.mnemonic.clone(),
      pubkey,
      name: Some(auth.user.display_name.clone()),
      email: auth.user.email.clone(),
      photo_url: Some(auth.user.photo_url.clone()),
    }).await?;

    // Push message to Rabbitmq
    store.new_user_queue.on_new_user(body.pubkey.clone()).await?;
  
    return Ok(HttpResponse::Created().finish())
  };

  // if account exist then simply return the account
  Ok(HttpResponse::Ok().json(account))
}
