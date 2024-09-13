use std::fs;

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::schemas::fs::GqlDirent;

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(description = "Dirent DTO")]
pub struct GqlDirentDTO {
    data: Option<GqlDirent>,
    error: Option<String>,
}

impl From<fs::DirEntry> for GqlDirentDTO {
    fn from(dirent: fs::DirEntry) -> Self {
        Self {
            data: Some(GqlDirent::from(dirent)),
            error: None,
        }
    }
}

impl From<std::io::Error> for GqlDirentDTO {
    fn from(err: std::io::Error) -> Self {
        Self {
            data: None,
            error: Some(err.to_string()),
        }
    }
}

impl From<Result<fs::DirEntry, std::io::Error>> for GqlDirentDTO {
    fn from(dirent_result: Result<fs::DirEntry, std::io::Error>) -> Self {
        match dirent_result {
            Ok(dirent) => Self::from(dirent),
            Err(err) => Self::from(err),
        }
    }
}
