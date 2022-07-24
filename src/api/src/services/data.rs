use std::sync::Arc;
use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use bolt_client::{Params};
use common::error::Error;
use data::{
  helpers::{send_read, send_write},
  types::{Neo4jResult},
};
use crate::utils::store::Store;
use api_helpers::services::http::internal_server_error;

/// Example `impl_query_string!(QueryString);`
#[macro_export]
macro_rules! impl_query_string {
  ($ty:ident) => {
    impl QueryStringTrait for $ty {
      fn skip(&self) -> Option<u32> { self.skip }
      fn limit(&self) -> Option<u32> { self.limit }
    }
  }
}

pub trait QueryStringTrait {
  fn skip(&self) -> Option<u32>;
  fn limit(&self) -> Option<u32>;
}

#[derive(Deserialize, Default)]
pub struct QueryString {
  pub skip: Option<u32>,
  pub limit: Option<u32>
}

impl_query_string!(QueryString);

#[derive(Serialize)]
pub struct BaseResponse {
  pub count: usize,
  pub skip: Option<u32>,
  pub limit: Option<u32>,
  pub result: Neo4jResult,
}

#[derive(Serialize)]
pub struct Neo4jBaseResponse {
  pub result: Neo4jResult,
}

pub type DbQueryBuilder = Box<dyn Fn() -> (&'static str, Option<Params>)>;

pub async fn exec_basic_db_read_endpoint(
  store: &web::Data<Store>,
  qs: Box<dyn QueryStringTrait>,
  db_query_builder: DbQueryBuilder
) -> HttpResponse {
  let skip = qs.skip().unwrap_or(0);
  let limit = qs.limit().unwrap_or(100);
  let (query, db_query_params) = db_query_builder();

  send_read(
    Arc::clone(&store.neo4j),
    query,
    db_query_params,
  ).await
  .map(|result| {
    HttpResponse::Ok()
      .json(BaseResponse {
        count: result.0.len(),
        result,
        skip: Some(skip),
        limit: Some(limit),
      })
  })
  .unwrap_or_else(|error: Error| internal_server_error(Some(error)))
}

pub async fn exec_basic_db_read_endpoint_no_qs(
  store: &web::Data<Store>,
  db_query_builder: DbQueryBuilder
) -> HttpResponse {
  let (query, db_query_params) = db_query_builder();

  send_read(
    Arc::clone(&store.neo4j),
    query,
    db_query_params,
  ).await
  .map(|result| {
    HttpResponse::Ok()
      .json(Neo4jBaseResponse {result})
  })
  .unwrap_or_else(|error: Error| internal_server_error(Some(error)))
}

pub async fn exec_basic_db_write_endpoint(
  store: &web::Data<Store>,
  db_query_builder: DbQueryBuilder
) -> HttpResponse {
  let (query, db_query_params) = db_query_builder();

  send_write(
    Arc::clone(&store.neo4j),
    query,
    db_query_params,
  ).await
  .map(|_| HttpResponse::Ok().finish())
  .unwrap_or_else(|error: Error| internal_server_error(Some(error)))
}
