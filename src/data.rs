use bincode2;
use cosmwasm_std::{
    to_binary, Api, Env, Extern, HandleResponse, Querier, QueryResult, ReadonlyStorage, StdError,
    StdResult, Storage,
};
use geohash::{encode, Coordinate};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::bucket::{load_all_buckets, Bucket, GeoLocationTime, Pointers};
use crate::msg::{GoogleTakeoutHistory, HotSpot, QueryAnswer};
use crate::trie::RecursiveTrie;
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

pub const DISTANCE: f64 = 10.0; // in meters
pub const EARTH_RADIUS: f64 = 6371000.0; // in meters
pub const OVERLAP_TIME: u64 = 1000 * 60 * 5;
pub const HOTSPOTS_ID: &[u8] = b"HOTSPOTS_ID";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HotSpots(pub Vec<HotSpot>);

impl HotSpots {
    pub fn from_vec(v: Vec<HotSpot>) -> Self {
        Self(v)
    }

    pub fn store<S: Storage>(&self, store: &mut S) -> StdResult<()> {
        let mut config_store = PrefixedStorage::new(HOTSPOTS_ID, store);
        let as_bytes =
            bincode2::serialize(&self).map_err(|_| StdError::generic_err("Error packing trie"))?;

        config_store.set(HOTSPOTS_ID, &as_bytes);

        Ok(())
    }

    pub fn load<S: Storage>(store: &S) -> StdResult<Self> {
        let config_store = ReadonlyPrefixedStorage::new(HOTSPOTS_ID, store);
        if let Some(trie) = config_store.get(HOTSPOTS_ID) {
            let ptrs: Self = bincode2::deserialize(&trie)
                .map_err(|_| StdError::generic_err("Error deserializing trie"))?;
            return Ok(ptrs);
        }

        Ok(Self::default())
    }
}

pub fn add_data_points<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data_points: Vec<GeoLocationTime>,
) -> StdResult<HandleResponse> {
    let pointers = Pointers::load(&deps.storage)?;

    for dp in data_points {
        if let Some(bucket) = pointers.find_bucket(dp.timestamp_ms) {
            let mut dis = Bucket::load(&deps.storage, &bucket)?;

            dis.insert_data_point(dp);

            dis.store(&mut deps.storage, &bucket)?;
        }
    }

    Ok(HandleResponse::default())
}

pub fn add_google_data<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data_points: GoogleTakeoutHistory,
) -> StdResult<HandleResponse> {
    let pointers = Pointers::load(&deps.storage)?;

    let mut buckets = load_all_buckets(&deps.storage)?;

    let mut trie = RecursiveTrie::load(&deps.storage)?;

    for dp in data_points.locations {
        if let Some(bucket) = pointers.find_bucket(dp.timestampMs.u128() as u64) {
            store_geohash(&mut trie, dp.hash()?);

            buckets
                .get_mut(&bucket)
                .unwrap()
                .insert_data_point(dp.into());
        }
    }

    for (name, b) in buckets {
        b.store(&mut deps.storage, &name)?;
    }

    let res = HotSpots::from_vec(
        cluster(&trie, 6, 10)
            .into_iter()
            .map(|kv| HotSpot {
                geo_location: kv.0,
                power: kv.1,
            })
            .collect(),
    );

    res.store(&mut deps.storage)?;

    trie.store(&mut deps.storage)?;

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

            let time_overlap = dis.search(dp.timestamp_ms, dp.timestamp_ms + OVERLAP_TIME);
            if time_overlap.len() > 0 {
                for point in time_overlap {
                    if match_location(&point, &dp) {
                        geo_overlap.push(point.clone());
                    }
                }
            }
        }
    }
    to_binary(&QueryAnswer::OverLap {
        data_ponts: geo_overlap,
    })
}

fn match_location(e: &GeoLocationTime, d: &GeoLocationTime) -> bool {
    if (e.lat - d.lat).abs() * 111000.0 < DISTANCE * 0.71 {
        // then we can run a more computationally expensive and precise comparison
        if (e.lat.sin() * d.lat.sin() + e.lat.cos() * d.lat.cos() * (e.lng - d.lng).cos()).acos()
            * EARTH_RADIUS
            < DISTANCE
        {
            return true;
        }
    }
    false
}

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

#[derive(Clone, Debug, Default)]
pub struct KeyVal(pub String, pub u32);

impl PartialOrd for KeyVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for KeyVal {
    fn eq(&self, other: &Self) -> bool {
        &self.1 == &other.1
    }
}
impl Ord for KeyVal {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1 > other.1 {
            true => Ordering::Greater,
            false => match self.1 == other.1 {
                true => Ordering::Equal,
                false => Ordering::Less,
            },
        }
    }
}

impl Eq for KeyVal {}

impl ToString for KeyVal {
    fn to_string(&self) -> String {
        return format!("{} : {}", self.0, self.1);
    }
}

pub fn cluster(t: &RecursiveTrie, depth: usize, zones: usize) -> Vec<KeyVal> {
    let mut hmap = HashMap::<String, u32>::new();
    let mut commons: Vec<KeyVal> = vec![];

    for _ in 0..zones {
        commons.push(KeyVal::default());
    }

    t.find_most_common(depth, &mut hmap);

    for (k, v) in hmap.iter() {
        if v > &commons.last().unwrap().1 {
            commons.pop();
            commons.push(KeyVal(k.clone(), v.clone()));
            commons.sort_unstable_by(|a, b| b.cmp(a));
        }
    }

    commons
}

fn store_geohash(mytrie: &mut RecursiveTrie, hash: String) {
    mytrie.insert(hash, 1)
}
