use serde::de;
use serde::{Serialize, Deserialize, Deserializer};

// Serialize and Clone all the data so we can process it later

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OfferData {
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: u32,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}

// Deserialize data from string to a float
pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

#[derive (Debug, Deserialize, Serialize, Clone)]
pub struct DepthStreamWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}
