use borsh::{BorshSerialize, BorshDeserialize};
use amqp_helpers::producer::retry_producer::RetryProducer;
use ticketland_signdrop::model::NewUser;
