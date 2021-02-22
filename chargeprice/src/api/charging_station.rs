use serde::Deserialize;

use super::{
    common::{EntityRef, InnerData, Response},
    plug::Plug,
};

#[derive(Debug, Deserialize)]
pub struct ChargingStationPoints {
    plug: Plug,
    power: f32,
    count: u32,
    available_count: u32,
}

impl ChargingStationPoints {
    /// Get a reference to the charging station points's plug.
    pub fn plug(&self) -> Plug {
        self.plug.clone()
    }

    /// Get a reference to the charging station points's power.
    pub fn power(&self) -> f32 {
        self.power.clone()
    }

    /// Get a reference to the charging station points's count.
    pub fn count(&self) -> u32 {
        self.count.clone()
    }

    /// Get a reference to the charging station points's available count.
    pub fn available_count(&self) -> u32 {
        self.available_count.clone()
    }
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

impl ChargingStationAttributes {
    /// Get a reference to the charging station attributes's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a reference to the charging station attributes's latitude.
    pub fn latitude(&self) -> f32 {
        self.latitude.clone()
    }

    /// Get a reference to the charging station attributes's longitude.
    pub fn longitude(&self) -> f32 {
        self.longitude.clone()
    }

    /// Get a reference to the charging station attributes's country.
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Get a reference to the charging station attributes's address.
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get a reference to the charging station attributes's free parking.
    pub fn free_parking(&self) -> Option<bool> {
        self.free_parking.clone()
    }

    /// Get a reference to the charging station attributes's charge points.
    pub fn charge_points(&self) -> &[ChargingStationPoints] {
        &self.charge_points
    }

    /// Get a reference to the charging station attributes's free charging.
    pub fn free_charging(&self) -> Option<bool> {
        self.free_charging.clone()
    }
}
#[derive(Debug, Deserialize)]
pub struct ChargingStationRelationShips {
    operator: InnerData<EntityRef>,
}

impl ChargingStationRelationShips {
    pub fn operator_id(&self) -> &str {
        &self.operator.id
    }
}

pub type ChargingStationResponse =
    Response<ChargingStationAttributes, ChargingStationRelationShips>;
