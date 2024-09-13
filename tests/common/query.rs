use graphql_client::GraphQLQuery;
// use reqwest;
// use rstest::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.graphql",
    query_path = "tests/graphql/ping.graphql",
    response_derives = "Debug"
)]
pub struct Ping;
