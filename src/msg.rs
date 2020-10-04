use crate::bucket::GeoLocationTime;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub start_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddDataPoints { data_points: Vec<GeoLocationTime> },
    ImportGoogleLocations { data: GoogleTakeoutHistory },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    MatchDataPoint { data_point: GeoLocationTime },
    HotSpot {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GoogleTakeoutHistory {
    pub locations: Vec<GoogleLocation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GoogleLocation {
    pub timestampMs: Uint128,
    pub latitudeE7: u64,
    pub longitudeE7: u64,
}

impl Into<GeoLocationTime> for GoogleLocation {
    fn into(self) -> GeoLocationTime {
        GeoLocationTime {
            lat: self.latitudeE7 as f64 / 1e7,
            lng: self.longitudeE7 as f64 / 1e7,
            timestamp_ms: self.timestampMs.u128() as u64,
        }
    }
}
