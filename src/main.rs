use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use tokio::{fs, time};

use crate::{config::Config, net2grid::Net2GridResponse};
mod net2grid;
mod config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf_str = fs::read_to_string("config.toml").await?;
    let config: Config = toml::from_str(&conf_str).expect("invalid config.");

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&config.database.url).await?;

    let mut interval = time::interval(Duration::from_secs(config.interval));
    println!("Started.");
    interval.tick().await;

    loop {
        let response: Net2GridResponse = match reqwest::get(&config.address).await {
            Ok(d) => {
                match d.json::<Net2GridResponse>().await {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("Error parsing JSON: {}", e);
                        interval.tick().await;
                        continue;
                    }
                }
            },
            Err(e) => {
                eprintln!("Error in request: {}", e);
                interval.tick().await;
                continue;
            }
        };

        match sqlx::query("INSERT INTO meter(time, power, consumption, production, gas_consumption) VALUES (NOW(), $1, $2, $3, $4);")
        .bind(response.elec.power.now.value as i32)
        .bind(response.elec.consumption.now.value)
        .bind(response.elec.production.now.value)
        .bind(response.gas.consumption.now.value)
        .execute(&pool).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error while executing database query: {e}");
            }
        };

        interval.tick().await;
    }
}