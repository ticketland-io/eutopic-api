use bolt_proto::value::{Value};
use bolt_client::{Params};
use common::{
  actor::neo4j::{create_params}
};

pub fn read_account(uid: String) -> (&'static str, Option<Params>) {
  let query = r#"
    MATCH (acc:Account)-[:REGISTERED]->(cke:CrossKycEmailRegistered)
    WITH apoc.agg.maxItems(cke, cke.block_number) AS result
    return COUNT(acc)
  "#;

  let params = create_params(vec![
    ("uid", Value::String(uid)),
  ]);


  (query, params)
}
