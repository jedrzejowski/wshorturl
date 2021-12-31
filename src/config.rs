use once_cell::sync::{Lazy};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppEnv {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub domain: Option<String>,

    pub db: Option<String>,
    pub yaml_file: Option<String>,
}

pub static APP_ENV: Lazy<AppEnv> = Lazy::new(|| {
    envy::from_env::<AppEnv>().unwrap()
});
