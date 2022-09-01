use std::env;

pub struct Config {
  pub port: u64,
  pub neo4j_host: String,
  pub neo4j_domain: Option<String>,
  pub neo4j_username: String,
  pub neo4j_password: String,
  pub neo4j_database: Option<String>,
  pub firebase_auth_key: String,
  pub cors_origin: Vec<String>,
  // Rabbitmq envs
  pub rabbitmq_uri: String,
  pub exchange_name: String,
  pub queue_name: String,
  pub routing_key: String,
  pub retry_ttl: u16,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        port: env::var("PORT").unwrap().parse::<u64>().unwrap(),
        neo4j_host: env::var("NEO4J_HOST").unwrap(),
        neo4j_domain: None,
        neo4j_username: env::var("NEO4J_USERNAME").unwrap(),
        neo4j_password: env::var("NEO4J_PASSWORD").unwrap(),
        neo4j_database: env::var("NEO4J_DATABASE").ok(),
        firebase_auth_key: env::var("FIREBASE_API_KEY").unwrap(),
        cors_origin: env::var("CORS_ORIGIN").unwrap().split(",").map(|val| val.to_owned()).collect(),
        rabbitmq_uri: env::var("RABBITMQ_URI").unwrap(),
        exchange_name: env::var("EXCHANGE_NAME").unwrap(),
        queue_name: env::var("QUEUE_NAME").unwrap(),
        routing_key: env::var("ROUTING_KEY").unwrap(),
        retry_ttl: env::var("RETRY_TTL").unwrap().parse::<u16>().unwrap(),
      }
    )
  }
}
