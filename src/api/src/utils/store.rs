use std::sync::{Arc, Mutex};
use ticketland_data::connection::PostgresConnection;
use crate::services::new_user_queue::NewUserQueue;

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub postgres: Arc<Mutex<PostgresConnection>>,
  pub new_user_queue: NewUserQueue,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();

    let postgres = Arc::new(Mutex::new(PostgresConnection::new(&config.postgres_uri).await));
    let new_user_queue = NewUserQueue::new(
      config.rabbitmq_uri.clone(),
      config.exchange_name.clone(),
      config.queue_name.clone(),
      config.routing_key.clone(),
      config.retry_ttl,
    ).await; 


    Self {
      config,
      postgres,
      new_user_queue,
    }
  }
}
