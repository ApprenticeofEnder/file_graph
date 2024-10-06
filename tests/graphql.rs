use actix_web::dev::ServerHandle;
use async_std::fs;
use gql_client::Client as GqlClient;
use rstest::*;

mod common;
use common::{
    query::{self, query_client, DirReadResponse, FileReadResponse, PathVariables, PingResponse},
    setup::test_server,
};
use file_graph::schemas;
use tempfile::{NamedTempFile, TempDir};

#[rstest]
#[tokio::test]
async fn test_ping(#[future] test_server: &ServerHandle, query_client: &GqlClient) {
    test_server.await;

    let query_path = std::path::Path::new("tests/graphql/ping.graphql");
    let query = fs::read_to_string(query_path).await.unwrap();
    println!("{}", query);
    let _res = query_client.query::<PingResponse>(&query).await.unwrap();
}

#[rstest]
#[case(true)]
#[case(false)]
#[tokio::test]
async fn test_file_read(
    #[case] file_exists: bool,
    #[future] test_server: &ServerHandle,
    query_client: &GqlClient,
) {
    test_server.await;
    let tmp_file = NamedTempFile::new().unwrap();
    let path: String = match file_exists {
        true => tmp_file
            .path()
            .as_os_str()
            .to_os_string()
            .to_string_lossy()
            .into_owned(),
        false => "/file/that/doesn't/exist".into(),
    };

    let variables = query::PathVariables { path };

    let query_path = std::path::Path::new("tests/graphql/read_file.graphql");
    let query = fs::read_to_string(query_path).await.unwrap();

    let res = query_client
        .query_with_vars::<FileReadResponse, PathVariables>(&query, variables)
        .await;

    match (res, file_exists) {
        (Ok(Some(_data)), false) => {
            panic!("Nonexistent files should return errors.");
        }
        (Err(err), true) => {
            panic!("{}", err.message());
        }
        _ => {}
    };
}

#[rstest]
#[case(true)]
#[case(false)]
#[tokio::test]
async fn test_dir_read(
    #[case] dir_exists: bool,
    #[future] test_server: &ServerHandle,
    query_client: &GqlClient,
) {
    test_server.await;
    let tmp_dir = TempDir::new().unwrap();
    let path: String = match dir_exists {
        true => tmp_dir
            .path()
            .as_os_str()
            .to_os_string()
            .to_string_lossy()
            .into_owned(),
        false => "/file/that/doesn't/exist".into(),
    };

    let variables = query::PathVariables { path };

    let query_path = std::path::Path::new("tests/graphql/read_dir.graphql");
    let query = fs::read_to_string(query_path).await.unwrap();

    let res = query_client
        .query_with_vars::<DirReadResponse, PathVariables>(&query, variables)
        .await;

    match (res, dir_exists) {
        (Ok(Some(_data)), false) => {
            panic!("Nonexistent directories should return errors.");
        }
        (Err(err), true) => {
            panic!("{}", err.message());
        }
        _ => {}
    };
}

// Todo: Add tests for read_home
