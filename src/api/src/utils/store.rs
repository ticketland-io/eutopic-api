use chrono::Duration;
use ticketland_data::connection_pool::ConnectionPool;
use crate::services::{
  new_user_queue::NewUserQueue,
  delete_account_request_queue::DeleteAccountRequestQueue,
};

use super::config::Config;

pub struct Store {
  pub config: Config,
  pub pg_pool: ConnectionPool,
  pub new_user_queue: NewUserQueue,
  pub delete_account_request_queue: DeleteAccountRequestQueue,
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
    let delete_account_request_queue = DeleteAccountRequestQueue::new(
      config.rabbitmq_uri.clone(),
      "delete_account_request".to_string(),
      "delete_account_request".to_string(),
      "delete_account_request.new".to_string(),
      config.retry_ttl,
      Some(Duration::days(14).num_milliseconds() as i32)
    ).await;

    Self {
      config,
      pg_pool,
      new_user_queue,
      delete_account_request_queue,
    }
  }
}
