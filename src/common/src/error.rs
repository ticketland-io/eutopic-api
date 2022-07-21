use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Neo4jError")]
  Neo4jError(String),
}
