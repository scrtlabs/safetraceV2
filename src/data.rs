use std::cmp::Ordering;
use std::convert::TryInto;

use crate::bucket::{load_all_buckets, Bucket, GeoLocationTime, Pointers};
use crate::hotspotmap::{HotSpots, HotspotMap};
use crate::msg::{GoogleTakeoutHistory, HotSpot, QueryAnswer};
use cosmwasm_std::{
    to_binary, Api, Env, Extern, HandleResponse, Querier, QueryResult, StdError, StdResult, Storage,
};
use geohash::{encode, Coordinate};

// pub const DISTANCE: f64 = 10.0; // in meters
// pub const EARTH_RADIUS: f64 = 6371000.0; // in meters
pub const OVERLAP_TIME: u64 = 1000 * 60 * 5;

pub fn add_google_data<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data_points: GoogleTakeoutHistory,
) -> StdResult<HandleResponse> {
    let pointers = Pointers::load(&deps.storage)?;

    let mut buckets = load_all_buckets(&deps.storage)?;

    let mut buck = HotspotMap::load(&deps.storage)?;

    for dp in data_points.locations {
        if let Some(bucket) = pointers.find_bucket(dp.timestampMs.u128() as u64) {
            let geopt: GeoLocationTime = dp.try_into()?;

            buck.insert_data_point(geopt.geohash.clone());

            buckets.get_mut(&bucket).unwrap().insert_data_point(geopt);
        }
    }

    let hotspots = HotSpots(buck.get_top_hotspots());

    for (name, b) in buckets {
        b.store(&mut deps.storage, &name)?;
    }

    buck.store(&mut deps.storage)?;

    hotspots.store(&mut deps.storage)?;

    Ok(HandleResponse::default())
}

pub fn match_data_point<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    data_point: Vec<GeoLocationTime>,
) -> QueryResult {
    let pointers = Pointers::load(&deps.storage)?;
    let mut geo_overlap: Vec<GeoLocationTime> = Vec::default();

    for dp in data_point {
        if let Some(bucket) = pointers.find_bucket(dp.timestamp_ms) {
            let dis = Bucket::load(&deps.storage, &bucket)?;

            if dis.match_pos(&dp.geohash, dp.timestamp_ms, OVERLAP_TIME) {
                geo_overlap.push(dp.clone());
            }
        }
    }
    to_binary(&QueryAnswer::OverLap {
        data_ponts: geo_overlap,
    })
}

// fn match_location(e: &GeoLocationTime, d: &GeoLocationTime) -> bool {
//     if (e.lat - d.lat).abs() * 111000.0 < DISTANCE * 0.71 {
//         // then we can run a more computationally expensive and precise comparison
//         if (e.lat.sin() * d.lat.sin() + e.lat.cos() * d.lat.cos() * (e.lng - d.lng).cos()).acos()
//             * EARTH_RADIUS
//             < DISTANCE
//         {
//             return true;
//         }
//     }
//     false
// }

pub fn ghash(x: f64, y: f64) -> StdResult<String> {
    encode(
        Coordinate {
            x, // lng
            y, // lat
        },
        9usize,
    )
    .map_err(|_| StdError::generic_err(format!("Cannot encode data to geohash ({}, {})", x, y)))
}

// #[derive(Clone, Debug, Default, Serialize, Deserialize)]
// pub struct KeyVal(pub String, pub u32);

impl PartialOrd for HotSpot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HotSpot {
    fn eq(&self, other: &Self) -> bool {
        &self.power == &other.power
    }
}
impl Ord for HotSpot {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.power > other.power {
            true => Ordering::Greater,
            false => match self.power == other.power {
                true => Ordering::Equal,
                false => Ordering::Less,
            },
        }
    }
}

impl Eq for HotSpot {}

impl ToString for HotSpot {
    fn to_string(&self) -> String {
        return format!("{} : {}", self.geo_location, self.power);
    }
}
