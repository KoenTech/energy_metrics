use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub interval: u64,
    pub database: Database
}

#[derive(Deserialize)]
pub struct Database {
    pub url: String
}