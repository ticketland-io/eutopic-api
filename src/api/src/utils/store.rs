use std::sync::Arc;
use actix::prelude::*;
use fireauth::FireAuth;
use common::actor::{neo4j::Neo4jActor};
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub firebase_auth: FireAuth,
  pub neo4j: Arc<Addr<Neo4jActor>>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();
    let firebase_auth = fireauth::FireAuth::new(config.firebase_auth_key.clone());

    let neo4j = Arc::new(
      Neo4jActor::new(
        config.neo4j_host.clone(),
        config.neo4j_domain.clone(),
        config.neo4j_username.clone(),
        config.neo4j_password.clone(),
        config.neo4j_database.clone(),
      )
      .await
      .start(),
    );

    Self {
      config,
      firebase_auth,
      neo4j,
    }
  }
}
