use borsh::{BorshSerialize, BorshDeserialize};
use amqp_helpers::producer::retry_producer::RetryProducer;
use eyre::Result;
use ticketland_event_handler::{
  models::account::DeleteAccountRequest,
};

pub struct DeleteAccountRequestQueue {
  producer: RetryProducer,
  exchange_name: String,
  routing_key: String,
}

impl DeleteAccountRequestQueue {
  pub async fn new(
    rabbitmq_uri: String,
    exchange_name: String,
    queue_name: String,
    routing_key: String,
    retry_ttl: u16,
    delay_ms: Option<i32>,
  ) -> Self {
    let producer = RetryProducer::new(
      &rabbitmq_uri,
      &exchange_name,
      &queue_name,
      &routing_key,
      retry_ttl,
      delay_ms,
    ).await.unwrap();

    Self {
      producer,
      exchange_name,
      routing_key,
    }
  }

  pub async fn on_new_delete_request(&self, uid: String) -> Result<()> {
    let msg = DeleteAccountRequest {
      uid
    };

    self.producer.publish(
      &self.exchange_name,
      &self.routing_key,
      &msg.try_to_vec().unwrap()
    ).await
  }
}
