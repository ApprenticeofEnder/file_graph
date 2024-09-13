use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.graphql",
    query_path = "tests/graphql/ping.graphql",
    response_derives = "Debug"
)]
pub struct Ping;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.graphql",
    query_path = "tests/graphql/read_file.graphql",
    response_derives = "Debug"
)]
pub struct ReadFile;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/graphql/schema.graphql",
    query_path = "tests/graphql/read_dir.graphql",
    response_derives = "Debug"
)]
pub struct ReadDir;
