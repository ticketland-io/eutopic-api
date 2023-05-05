use std::env;

pub struct Config {
  pub port: u64,
  pub postgres_uri: String,
  pub firebase_auth_key: String,
  pub cors_origin: Vec<String>,
  // Rabbitmq envs
  pub rabbitmq_uri: String,
  pub exchange_name: String,
  pub queue_name: String,
  pub routing_key: String,
  pub retry_ttl: u32,
  pub enc_key: Vec<u8>,
  pub account_deletion_delay_days: i64,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        port: env::var("PORT").unwrap().parse::<u64>().unwrap(),
        postgres_uri: env::var("POSTGRES_URI").unwrap(),
        firebase_auth_key: env::var("FIREBASE_API_KEY").unwrap(),
        cors_origin: env::var("CORS_ORIGIN").unwrap().split(",").map(|val| val.to_owned()).collect(),
        rabbitmq_uri: env::var("RABBITMQ_URI").unwrap(),
        exchange_name: env::var("EXCHANGE_NAME").unwrap(),
        queue_name: env::var("QUEUE_NAME").unwrap(),
        routing_key: env::var("ROUTING_KEY").unwrap(),
        retry_ttl: env::var("RETRY_TTL").unwrap().parse::<u32>().unwrap(),
        enc_key: env::var("ENC_KEY").unwrap().as_bytes().to_vec(),
        account_deletion_delay_days: env::var("ACCOUNT_DELETION_DELAY_DAYS").unwrap().parse::<i64>().unwrap(),
      }
    )
  }
}
