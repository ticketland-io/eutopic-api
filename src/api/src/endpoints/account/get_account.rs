use eyre::Result;
use actix_web::{web, HttpResponse};
use ticketland_core::error::Error;
use api_helpers::{
  middleware::auth::AuthData,
};
use ticketland_crypto::symetric::aes::{decrypt};
use crate::{
  utils::store::Store,
};

pub async fn exec(
  store: web::Data<Store>,
  auth: AuthData,
) -> Result<HttpResponse, Error> {
  let mut postgres = store.pg_pool.connection().await?;
  let Ok(mut account) = postgres.read_account_by_id(auth.user.local_id).await else {
    return Ok(HttpResponse::NotFound().finish())
  };

  let nonce = &account.pubkey.as_bytes()[..12];
  account.dapp_share = decrypt(
    &store.config.enc_key[..32],
    nonce,
    account.dapp_share.as_ref(),
  )?;

  Ok(HttpResponse::Ok().json(account))
}
