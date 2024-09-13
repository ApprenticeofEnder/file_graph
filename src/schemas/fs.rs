use std::{ffi::OsString, fs};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use juniper::{GraphQLEnum, GraphQLObject};
use num::ToPrimitive;
use serde::{Deserialize, Serialize};

pub fn osstring_to_string(path: &OsString) -> String {
    return path.to_string_lossy().into_owned();
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "Dirent")]
pub struct GqlDirent {
    path: String,
    metadata: Option<GqlFileMetadata>,
    #[serde(rename = "fileName")]
    file_name: String,
}

impl From<fs::DirEntry> for GqlDirent {
    fn from(dirent: fs::DirEntry) -> Self {
        let path = osstring_to_string(&dirent.path().into_os_string());
        let file_name = osstring_to_string(&dirent.file_name());

        let metadata: Option<GqlFileMetadata> = match dirent.metadata() {
            Ok(metadata) => Some(GqlFileMetadata::from(metadata)),
            Err(err) => {
                log::error!("Error processing metadata: {}", err.to_string());
                None
            }
        };
        GqlDirent {
            path,
            metadata,
            file_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "File")]
pub struct GqlFile {
    contents: String,
    #[serde(rename = "fileName")]
    file_name: String,
    metadata: Option<GqlFileMetadata>,
    path: String,
}

impl GqlFile {
    pub fn new(path: &String) -> Result<Self, std::io::Error> {
        let full_path = fs::canonicalize(path)?;
        let contents: String = match fs::read_to_string(&full_path) {
            Ok(contents) => contents,
            Err(err) => {
                log::error!("Unable to read file: {}", err.to_string());
                let raw_data = fs::read(&full_path)?;
                URL_SAFE.encode(raw_data)
            }
        };

        let file_name = match full_path.file_name() {
            Some(name) => osstring_to_string(&name.into()),
            None => {
                log::warn!("Unable to retrieve file name for {}.", path);
                "Unknown".into()
            }
        };

        let metadata: Option<GqlFileMetadata> = match fs::metadata(&full_path) {
            Ok(metadata) => Some(GqlFileMetadata::from(metadata)),
            Err(err) => {
                log::error!(
                    "Error processing metadata for {}: {}",
                    path,
                    err.to_string()
                );
                None
            }
        };

        let path = osstring_to_string(&full_path.into_os_string());

        Ok(Self {
            contents,
            file_name,
            metadata,
            path,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, GraphQLEnum)]
enum GqlFileType {
    File,
    Directory,
    Symlink,
    Unsupported,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "Metadata")]
pub struct GqlFileMetadata {
    #[serde(rename = "fileType")]
    file_type: GqlFileType,
    size_kb: i32,
    #[serde(rename = "readOnly")]
    read_only: bool,
    modified: f64,
    // TODO: This is going to be a problem in 2038. No easy fix for now.
    accessed: f64,
    created: f64,
}

fn system_time_to_secs(
    system_time: Result<std::time::SystemTime, std::io::Error>,
) -> Result<f64, std::io::Error> {
    let result = system_time?
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    Ok(result)
}

impl From<std::fs::Metadata> for GqlFileMetadata {
    fn from(metadata: std::fs::Metadata) -> Self {
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
