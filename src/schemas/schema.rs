use std::fs;

use juniper::{
    graphql_value, EmptyMutation, EmptySubscription, FieldError, FieldResult, GraphQLObject,
    RootNode,
};

use crate::schemas::dto::GqlDirentDTO;
use crate::schemas::fs::GqlFile;

use super::fs::osstring_to_string;

#[derive(GraphQLObject)]
#[graphql(description = "Ping")]
struct Ping {
    pong: String,
}
pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn ping() -> FieldResult<Ping> {
        Ok(Ping {
            pong: "Pong!".to_string(),
        })
    }

    fn read_dir(path: String) -> FieldResult<Vec<GqlDirentDTO>> {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(err) => {
                let error = FieldError::new(
                    format!("Could not retrieve directory: {}.", err),
                    graphql_value!({"error": "Directory not found"}),
                );
                return Err(error);
            }
        };
        let entries: Vec<GqlDirentDTO> = dir.map(GqlDirentDTO::from).collect();
        Ok(entries)
    }

    fn read_file(path: String) -> FieldResult<GqlFile> {
        let file = match GqlFile::new(&path) {
            Ok(file) => file,
            Err(err) => {
                let err = FieldError::new(
                    format!("Could not retrieve file: {}.", err),
                    graphql_value!({"error": "File not found"}),
                );
                return Err(err);
            }
        };
        Ok(file)
    }

    fn read_home() -> FieldResult<Vec<GqlDirentDTO>> {
        let home_dir: String = match home::home_dir() {
            Some(dir) => osstring_to_string(&dir.as_os_str().to_os_string()),
            None => {
                let err = FieldError::new(
                    "Could not retrieve home directory.".to_string(),
                    graphql_value!({"error": "Directory not found"}),
                );
                return Err(err);
            }
        };
        Self::read_dir(home_dir)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
