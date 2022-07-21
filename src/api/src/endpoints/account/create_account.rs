use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use serde_json;
use crate::{
  utils::store::Store,
  services::api_helpers::unauthorized_error,
};

#[derive(Deserialize)]
pub struct Body {
  firebase_token_id: String,
}

pub async fn exec(store: web::Data<Store>, body: web::Json<Body>) -> HttpResponse {
  match store.firebase_auth.get_user_info(&body.firebase_token_id).await {
    Ok(user) => {
      HttpResponse::Ok().body(serde_json::to_string(&user).expect("unexpected user data received from firebase"))
    },
    Err(error) => {
      println!("{:?}", error);
      unauthorized_error()
    },
  }
}
