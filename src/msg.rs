use std::convert::TryInto;

use cosmwasm_std::{HumanAddr, StdError, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::geohash::{ghash, GeoLocationTime};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub start_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// Adds new data to the contract, in the format specified by `GoogleTakeoutHistory`.
    ImportGoogleLocations { data: GoogleTakeoutHistory },
    /// ChangeDay is used to signal the contract that a day has passed, and all the oldest data,
    /// which pertains to 14 days ago is now invalid, and should be removed. This function may take
    /// a while, depending on how much data is stored in the contract
    ChangeDay {},
    /// Admins have permissions to import data and invalidate old data
    /// This function adds a new admin which can manage the contract
    AddAdmin { address: HumanAddr },
    /// Admins have permissions to import data and invalidate old data
    /// This function removes an admin. Any admin can remove and other admin -
    /// consider customizing this functionality according to access control policies
    RemoveAdmin { address: HumanAddr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// This query returns all the data points from the input which overlap with data stored
    /// in the contract. Aka, all the points that overlap in both location and time, to the accuracy
    /// defined by the contract (10 meter/5 minutes by default)
    MatchDataPoints { data_points: Vec<GoogleLocation> },
    /// This query returns the 10 most active zone, accurate to about a ~70m radius
    HotSpot {
        /// unused
        accuracy: Option<u32>,
        /// unused
        zones: Option<u32>,
    },
    /// Returns the earliest and latest times allowed by the contract for data storage
    TimeRange {},
}

/// General structure for query responses. All responses are returned as snake_case JSON objects
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    Overlap { data_points: Vec<GeoLocationTime> },
    HotSpotResponse { hot_spots: Vec<HotSpot> },
    DateRange { from: u64, to: u64 },
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, JsonSchema)]
pub struct HotSpot {
    pub geo_location: String,
    pub power: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GoogleTakeoutHistory {
    pub locations: Vec<GoogleLocation>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GoogleLocation {
    pub timestampMs: Uint128,
    pub latitudeE7: u64,
    pub longitudeE7: u64,
}

impl TryInto<GeoLocationTime> for GoogleLocation {
    type Error = StdError;

    fn try_into(self) -> StdResult<GeoLocationTime> {
        let geohash = self.hash().map_err(|_| {
            StdError::generic_err(format!(
                "failed to create geohash for ({}, {})",
                self.longitudeE7, self.latitudeE7
            ))
        })?;
        Ok(GeoLocationTime {
            geohash,
            timestamp_ms: self.timestampMs.u128() as u64,
        })
    }
}

impl GoogleLocation {
    pub fn hash(&self) -> StdResult<String> {
        ghash(self.longitudeE7 as f64 / 1e7, self.latitudeE7 as f64 / 1e7)
    }
}
