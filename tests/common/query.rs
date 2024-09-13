use crate::schemas::fs::GqlFile;

use file_graph::schemas::{self, dto::GqlDirentDTO};
use gql_client::Client;
use rstest::fixture;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PathVariables {
    pub path: String,
}

#[derive(Deserialize, Serialize)]
pub struct PingResponse {
    ping: schemas::schema::Ping,
}

#[derive(Deserialize, Serialize)]
pub struct FileReadResponse {
    #[serde(rename = "readFile")]
    read_file: GqlFile,
}

#[derive(Deserialize, Serialize)]
pub struct DirReadResponse {
    #[serde(rename = "readDir")]
    read_dir: Vec<GqlDirentDTO>,
}

#[fixture]
#[once]
pub fn query_client() -> Client {
    let client = Client::new("http://localhost:8080/graphql");
    client
}
