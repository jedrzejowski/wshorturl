use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use serde::{Deserialize};
use anyhow::Result;
use crate::database::DatabaseError;
use super::{Database, DatabaseTrait, UrlEntry};

#[derive(Clone, Deserialize)]
pub struct YamlRecordRow {
    url: String,
}

#[derive(Clone, Deserialize)]
pub struct YamlFileDatabase {
    urls: HashMap<String, YamlRecordRow>,
}

impl DatabaseTrait for YamlFileDatabase {
    fn get_url_entry(&self, url_id: &str) -> Result<UrlEntry> {
        let entry = self.urls.get(url_id);

        match &entry {
            Some(row) => {
                return Ok(UrlEntry {
                    url_id: url_id.to_string(),
                    url: row.url.clone(),
                });
            }
            None => {
               return Err(DatabaseError::NotFound {
                   url_id: url_id.to_string()
               }.into())
            }
        }
    }
}

impl YamlFileDatabase {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<YamlFileDatabase> {
        let file = std::fs::File::open(path)?;
        let database: YamlFileDatabase = serde_yaml::from_reader(file)?;

        return Ok(database);
    }
}

impl Into<Database> for YamlFileDatabase {
    fn into(self) -> Database {
        Arc::new(self)
    }
}