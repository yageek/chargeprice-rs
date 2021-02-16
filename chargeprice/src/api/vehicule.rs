use serde::Deserialize;

use super::common::{EntityRef, InnerData, Response};
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VehiculeChargePort {
    #[serde(rename(deserialize = "ccs"))]
    CCS,
    #[serde(rename(deserialize = "tesla_ccs"))]
    TeslaCCS,
    #[serde(rename(deserialize = "chademo"))]
    CHADemo,
    #[serde(rename(deserialize = "tesla_suc"))]
    TeslaSUC,
}

/// The attributes of the vehicules (cf: https://github.com/chargeprice/chargeprice-api-docs/blob/master/api/v1/vehicles/index.md)
#[derive(Debug, Deserialize)]
pub struct VehiculeAttributes {
    name: String,
    brand: String,
    dc_charge_ports: Vec<VehiculeChargePort>,
}

impl VehiculeAttributes {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn brand(&self) -> &str {
        &self.brand
    }
}
/// The relationships of the vehicules (cf: https://github.com/chargeprice/chargeprice-api-docs/blob/master/api/v1/vehicles/index.md)
#[derive(Debug, Deserialize)]
pub struct VehiculeRelationships {
    manufacturer: InnerData<EntityRef>,
}

impl VehiculeRelationships {
    pub fn manufacturer_id(&self) -> &str {
        &self.manufacturer.id
    }
}

/// The payload object returns when asking for vehicules (cf: https://github.com/chargeprice/chargeprice-api-docs/blob/master/api/v1/vehicles/index.md)
pub type VehiculeResponse = Response<VehiculeAttributes, VehiculeRelationships>;
