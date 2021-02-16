use serde::Deserialize;

use super::common::{EntityRef, InnerData, Response};
#[derive(Debug, Deserialize)]
pub struct ChargingStationPoints {
    plug: String,
    power: f32,
    count: u32,
    available_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChargingStationAttributes {
    name: String,
    latitude: f32,
    longitude: f32,
    country: String,
    address: String,
    free_parking: Option<bool>,
    free_charging: Option<bool>,
    charge_points: Vec<ChargingStationPoints>,
}

#[derive(Debug, Deserialize)]
pub struct ChargingStationRelationShips {
    operator: InnerData<EntityRef>,
}

pub type ChargingStationResponse =
    Response<ChargingStationAttributes, ChargingStationRelationShips>;
