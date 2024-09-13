use actix_web::dev::ServerHandle;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{self, RequestBuilder};
use rstest::*;

mod common;
use common::{
    query::{self, ping, read_file},
    setup::{req, test_server},
};
use tempfile::{NamedTempFile, TempDir};

#[rstest]
#[tokio::test]
async fn test_ping(#[future] _test_server: &ServerHandle, req: RequestBuilder) {
    _test_server.await;

    let variables = query::ping::Variables {};
    let request_body = query::Ping::build_query(variables);
    let res = req.json(&request_body).send().await.unwrap();
    let response_body: Response<ping::ResponseData> = res.json().await.unwrap();
    assert_eq!(response_body.data.unwrap().ping.pong, "Pong!");
}

#[rstest]
#[case(true)]
#[case(false)]
#[tokio::test]
async fn test_file_read(
    #[case] file_exists: bool,
    #[future] _test_server: &ServerHandle,
    req: RequestBuilder,
) {
    _test_server.await;
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

    let variables = query::read_file::Variables { path: path.clone() };
    let request_body = query::ReadFile::build_query(variables);
    let res = req.json(&request_body).send().await.unwrap();
    let response_body: Response<read_file::ResponseData> = res.json().await.unwrap();

    assert!(response_body.data.is_some() == file_exists);
    assert!(response_body.errors.is_none() == file_exists);
    match response_body {
        Response {
            data: Some(file_data),
            errors: None,
            ..
        } => {
            assert!(&file_data.read_file.path == &path);
            assert!(file_exists);
        }
        Response {
            data: None,
            errors: Some(file_errors),
            ..
        } => file_errors.iter().for_each(|err| {
            assert!(
                &err.message == "Could not retrieve file: No such file or directory (os error 2)."
            );
            assert!(!file_exists);
        }),
        _ => {
            panic!("Unknown case encountered. {:?}", response_body);
        }
    }
}

#[rstest]
#[case(true)]
#[case(false)]
#[tokio::test]
async fn test_dir_read(
    #[case] dir_exists: bool,
    #[future] _test_server: &ServerHandle,
    req: RequestBuilder,
) {
    use common::query::read_dir;

    _test_server.await;
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

    let variables = query::read_dir::Variables { path: path.clone() };
    let request_body = query::ReadDir::build_query(variables);
    let res = req.json(&request_body).send().await.unwrap();
    let response_body: Response<read_dir::ResponseData> = res.json().await.unwrap();

    assert!(response_body.data.is_some() == dir_exists);
    assert!(response_body.errors.is_none() == dir_exists);
    match response_body {
        Response {
            data: Some(dir_data),
            errors: None,
            ..
        } => {
            assert!(dir_exists);
            dir_data
                .read_dir
                .iter()
                .for_each(|dirent_dto| match &dirent_dto.data {
                    Some(dirent) => {
                        assert!(dirent.path == path);
                    }
                    None => panic!(
                        "Dirent could not be opened. {}",
                        dirent_dto.error.as_ref().unwrap()
                    ),
                });
        }
        Response {
            data: None,
            errors: Some(file_errors),
            ..
        } => file_errors.iter().for_each(|err| {
            assert!(
                &err.message
                    == "Could not retrieve directory: No such file or directory (os error 2)."
            );
            assert!(!dir_exists);
        }),
        _ => {
            panic!("Unknown case encountered. {:?}", response_body);
        }
    }
}

// Todo: Add tests for read_home
