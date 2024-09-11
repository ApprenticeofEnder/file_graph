use std::{
    fs::{self, Metadata},
    time::SystemTime,
};

use juniper::{
    graphql_value, EmptyMutation, EmptySubscription, FieldError, FieldResult, GraphQLEnum,
    GraphQLObject, GraphQLUnion, RootNode,
};
use num::ToPrimitive;

#[derive(GraphQLObject)]
#[graphql(description = "Ping")]
struct Ping {
    pong: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "Dirent")]
struct GqlDirent {
    path: String,
    metadata: GqlFileMetadata,
    file_name: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "Inaccessible Dirent")]
struct GqlBadDirent {
    error: String,
}

#[derive(GraphQLUnion)]
enum GqlDirentOption {
    GqlDirent(GqlDirent),
    GqlBadDirent(GqlBadDirent),
}

#[derive(GraphQLEnum)]
enum GqlFileType {
    File,
    Directory,
    Symlink,
    Unsupported,
}

#[derive(GraphQLObject)]
#[graphql(description = "Metadata")]
struct GqlFileMetadata {
    file_type: GqlFileType,
    size_kb: i32,
    read_only: bool,
    modified: f64,
    // TODO: This is going to be a problem in 2038. No easy fix for now.
    accessed: f64,
    created: f64,
}

fn system_time_to_secs(
    system_time: Result<SystemTime, std::io::Error>,
) -> Result<f64, std::io::Error> {
    let result = system_time?
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    Ok(result)
}

impl From<Metadata> for GqlFileMetadata {
    fn from(metadata: Metadata) -> Self {
        let file_type_obj = metadata.file_type();
        let file_type: GqlFileType;
        if file_type_obj.is_dir() {
            file_type = GqlFileType::Directory;
        } else if file_type_obj.is_file() {
            file_type = GqlFileType::File;
        } else if file_type_obj.is_symlink() {
            file_type = GqlFileType::Symlink;
        } else {
            file_type = GqlFileType::Unsupported;
        }
        let read_only: bool = metadata.permissions().readonly();
        let size_kb: i32 = (metadata.len() / 1024).to_i32().unwrap();
        let modified = system_time_to_secs(metadata.modified()).unwrap();
        let accessed = system_time_to_secs(metadata.accessed()).unwrap();
        let created = system_time_to_secs(metadata.created()).unwrap();
        Self {
            file_type,
            read_only,
            size_kb,
            modified,
            accessed,
            created,
        }
    }
}
/* How do you represent a file?
- Rust's fs module
- DirEntry
- File
- ReadDir
*/
pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn ping() -> FieldResult<Ping> {
        Ok(Ping {
            pong: "Pong!".to_string(),
        })
    }

    fn read_dir() -> FieldResult<Vec<GqlDirentOption>> {
        let Ok(dir) = fs::read_dir(".") else {
            return Err(FieldError::new(
                "Could not retrieve directory.",
                graphql_value!({"error": "Directory not found"}),
            ));
        };
        let entries: Vec<GqlDirentOption> = dir
            .map(|dirent| match dirent {
                Ok(entry) => {
                    let path = entry.path().to_string_lossy().into_owned();
                    let file_name = entry.file_name().to_string_lossy().into_owned();

                    let Ok(metadata) = entry.metadata() else {
                        return GqlDirentOption::GqlBadDirent(GqlBadDirent {
                            error: "Could not read file metadata".to_string(),
                        });
                    };

                    GqlDirentOption::GqlDirent(GqlDirent {
                        path,
                        metadata: GqlFileMetadata::from(metadata),
                        file_name,
                    })
                }
                Err(err) => GqlDirentOption::GqlBadDirent(GqlBadDirent {
                    error: err.to_string(),
                }),
            })
            .collect();
        Ok(entries)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
