use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerInfo {
    pub value: i64,
    pub unit: String,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Power {
    pub now: PowerInfo,
    pub min: PowerInfo,
    pub max: PowerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElecConsumption {
    pub now: PowerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElecProduction {
    pub now: PowerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Elec {
    pub power: Power,
    pub consumption: ElecConsumption,
    pub production: ElecProduction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GasConsumption {
    pub now: PowerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gas {
    pub consumption: GasConsumption,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Net2GridResponse {
    pub status: String,
    pub elec: Elec,
    pub gas: Gas,
}