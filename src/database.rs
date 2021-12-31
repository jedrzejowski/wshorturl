pub mod yaml_database;

use std::sync::Arc;
use serde::{Serialize};
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct UrlEntry {
    pub url: String,
    pub url_id: String,
}

pub trait DatabaseTrait: Send + Sync {
    fn get_url_entry(&self, url_id: &str) -> Result<UrlEntry>;
}

pub type Database = Arc<dyn DatabaseTrait + 'static>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("404 url not found")]
    NotFound {
        url_id: String,
    },
}