use ticketland_data::connection_pool::ConnectionPool;
use crate::services::new_user_queue::NewUserQueue;

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub pg_pool: ConnectionPool,
  pub new_user_queue: NewUserQueue,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();

    let pg_pool = ConnectionPool::new(&config.postgres_uri).await;
    let new_user_queue = NewUserQueue::new(
      config.rabbitmq_uri.clone(),
      config.exchange_name.clone(),
      config.queue_name.clone(),
      config.routing_key.clone(),
      config.retry_ttl,
    ).await; 


    Self {
      config,
      pg_pool,
      new_user_queue,
    }
  }
}
