use serde::Deserialize;
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Plug {
    #[serde(rename(deserialize = "ccs"))]
    CCS,
    #[serde(rename(deserialize = "tesla_ccs"))]
    TeslaCCS,
    #[serde(rename(deserialize = "chademo"))]
    CHADemo,
    #[serde(rename(deserialize = "tesla_suc"))]
    TeslaSUC,
    #[serde(rename(deserialize = "schuko"))]
    Schuko,
    #[serde(rename(deserialize = "type1"))]
    Type1,
    #[serde(rename(deserialize = "type2"))]
    Type2,
    #[serde(rename(deserialize = "type3"))]
    Type3,
}
