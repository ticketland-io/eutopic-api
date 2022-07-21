use fireauth::FireAuth;
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub firebase_auth: FireAuth,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();
    let firebase_auth = fireauth::FireAuth::new(config.firebase_auth_key.clone());

    Self {
      config,
      firebase_auth,
    }
  }
}
