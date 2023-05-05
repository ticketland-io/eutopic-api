use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use ticketland_core::error::Error;
use ticketland_crypto::symetric::aes::{encrypt};
use ticketland_data::models::account::Account;
use api_helpers::{
  middleware::auth::AuthData,
};
use crate::{
  utils::store::Store,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
  dapp_share: String,
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
    let nonce = pubkey.as_bytes();
    let dapp_share = encrypt(
      &store.config.enc_key[..32],
      &nonce[..12],
      body.dapp_share.as_bytes(),
    )?;
    
    postgres.upsert_account(Account {
      uid,
      created_at: None,
      dapp_share,
      pubkey,
      name: auth.user.display_name.clone(),
      email: auth.user.email.clone(),
      photo_url: auth.user.photo_url.clone(),
      delete_request_at: None,
      deleted_at: None,
    }).await?;

    // Push message to Rabbitmq
    store.new_user_queue.on_new_user(body.pubkey.clone()).await?;
  
    return Ok(HttpResponse::Created().finish())
  };

  // if account exist then simply return the account
  Ok(HttpResponse::Ok().json(account))
}
