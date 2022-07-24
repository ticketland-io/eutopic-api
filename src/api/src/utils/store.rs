use std::sync::Arc;
use actix::prelude::*;
use common::actor::{neo4j::Neo4jActor};
use crate::services::new_user_queue::NewUserQueue;

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub neo4j: Arc<Addr<Neo4jActor>>,
  pub new_user_queue: NewUserQueue,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();
    let new_user_queue = NewUserQueue::new(
      config.rabbitmq_uri.clone(),
      config.exchange_name.clone(),
      config.queue_name.clone(),
      config.routing_key.clone(),
      config.retry_ttl,
    ).await; 

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
      neo4j,
      new_user_queue,
    }
  }
}
