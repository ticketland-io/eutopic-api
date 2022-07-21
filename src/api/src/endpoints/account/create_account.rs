use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use data::{
  repositories::account::upsert_account,
};
use crate::{
  utils::store::Store,
  services::{
    api_helpers::unauthorized_error,
    data::exec_basic_db_write_endpoint,
  },
};

#[derive(Deserialize)]
pub struct Body {
  firebase_token_id: String,
  mnemonic: String,
}

pub async fn exec(store: web::Data<Store>, body: web::Json<Body>) -> HttpResponse {
  match store.firebase_auth.get_user_info(&body.firebase_token_id).await {
    Ok(user) => {
      exec_basic_db_write_endpoint(
        store,
        Box::new(move || {
          upsert_account(
            user.local_id.clone(),
            body.mnemonic.clone(),
          )
        })
      ).await;

      HttpResponse::Created().finish()
    },
    Err(_) => {
      unauthorized_error()
    },
  }
}
