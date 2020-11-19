use cosmwasm_std::{StdError, StdResult};
use geohash::{encode, Coordinate};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const PRECISION: usize = 9usize;

/// return the geohash to a precision degree specified by `PRECISION`.
/// 7 ~ 76m
/// 8 ~ 20m
/// 9 ~ 7m
/// 10 ~ 1m
pub fn ghash(x: f64, y: f64) -> StdResult<String> {
    encode(
        Coordinate {
            x, // lng
            y, // lat
        },
        PRECISION,
    )
    .map_err(|_| StdError::generic_err(format!("Cannot encode data to geohash ({}, {})", x, y)))
}

pub fn neighbors(geohash: &String) -> StdResult<Vec<String>> {
    let mut all: Vec<String> = vec![];

    let positions = geohash::neighbors(geohash)
        .map_err(|_| StdError::generic_err("Failed to decode geohash"))?;

    all.push(positions.n);
    all.push(positions.ne);
    all.push(positions.e);
    all.push(positions.se);
    all.push(positions.s);
    all.push(positions.sw);
    all.push(positions.w);
    all.push(positions.nw);

    Ok(all)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GeoLocationTime {
    pub geohash: String,
    pub timestamp_ms: u64,
}
