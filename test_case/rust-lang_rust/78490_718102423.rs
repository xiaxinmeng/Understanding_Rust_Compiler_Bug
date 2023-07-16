rust
use graphql_client::*;

#[allow(missing_docs)]
#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/query/schema/get.graphql",
    schema_path = "schema.graphql",
    response_derives = "PartialEq, Debug, Serialize, Deserialize",
    deprecated = "warn"
)]
/// Documentation for this struct
pub struct GetSchemaQuery;
