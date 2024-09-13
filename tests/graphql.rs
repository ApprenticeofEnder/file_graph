// async fn perform_my_query(variables: union_query::Variables) -> Result<(), Box<dyn Error>> {
//     // this is the important line
//     let request_body = union_query::build_query(variables);

//     let client = reqwest::Client::new();
//     let mut res = client.post("/graphql").json(&request_body).send().await?;
//     let response_body: Response<union_query::ResponseData> = res.json().await?;
//     println!("{:#?}", response_body);
//     Ok(())
// }

use graphql_client::{GraphQLQuery, Response};
use reqwest;
use rstest::*;

mod common;
use common::{
    query::{self, ping},
    setup::{run_test, setup},
};

#[rstest]
async fn test_ping(_setup: ()) {
    run_test(|| {
        Box::pin(async move {
            let variables = query::ping::Variables {};
            let request_body = query::Ping::build_query(variables);
            let client = reqwest::Client::new();
            let res = client
                .post("http://localhost:8080/graphql")
                .json(&request_body)
                .send()
                .await
                .unwrap();
            let response_body: Response<ping::ResponseData> = res.json().await.unwrap();
            assert_eq!(response_body.data.unwrap().ping.pong, "Pong!");
        })
    })
}
