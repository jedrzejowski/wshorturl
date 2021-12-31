use once_cell::sync::{Lazy, OnceCell};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppEnv {
    pub host: Option<String>,
    pub port: Option<i32>,
    pub domain: Option<String>,

    pub db: String,
    pub yaml_file: Option<String>,
}

pub static APP_ENV: Lazy<AppEnv> = Lazy::new(|| {
    envy::from_env::<AppEnv>().unwrap()
});
