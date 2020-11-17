use std::collections::HashMap;
use std::slice::Iter;

use bincode2;
use cosmwasm_std::{ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::geohash::{neighbors, GeoLocationTime};
use crate::pointer::{Pointer, Pointers, ONE_DAY};

use self::BucketName::*;

pub static BUCKETS_KEY: &[u8] = b"buckets";

/// `DailyBucket` stores all the geolocation data for a single day. It is not aware of any limits
/// itself. That is handled by the `Pointer` struct, which we use to select the appropriate bucket
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DailyBucket {
    // optionally - store by time->location. Ends up requiring much more storage, since time resolution
    // is higher than location resolution. Storing in a BTreeMap makes searching for time ranges easier.
    // pub locations: BTreeMap<u64, Locations>,
    pub locations: HashMap<String, Times>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum BucketName {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
}

impl BucketName {
    pub fn iterator() -> Iter<'static, BucketName> {
        static DIRECTIONS: [BucketName; 14] = [
            One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Eleven, Twelve, Thirteen,
            Fourteen,
        ];
        DIRECTIONS.iter()
    }
}

impl Into<&[u8]> for BucketName {
    fn into(self) -> &'static [u8] {
        match self {
            One => b"One",
            Two => b"Two",
            Three => b"Three",
            Four => b"Four",
            Five => b"Five",
            Six => b"Six",
            Seven => b"Seven",
            Eight => b"Eight",
            Nine => b"Nine",
            Ten => b"Ten",
            Eleven => b"Eleven",
            Twelve => b"Twelve",
            Thirteen => b"Thirteen",
            Fourteen => b"Fourteen",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Times(pub Vec<u64>);

impl Default for Times {
    fn default() -> Self {
        let this: Vec<u64> = vec![];

        return Self { 0: this };
    }
}

impl DailyBucket {
    pub fn store<S: Storage>(&self, store: &mut S, id: &BucketName) -> StdResult<()> {
        let mut config_store = PrefixedStorage::new(BUCKETS_KEY, store);
        let as_bytes = bincode2::serialize(&self)
            .map_err(|_| StdError::generic_err("Error packing pointers"))?;

        config_store.set((*id).into(), &as_bytes);

        Ok(())
    }

    pub fn load<S: Storage>(store: &S, id: &BucketName) -> StdResult<Self> {
        let config_store = ReadonlyPrefixedStorage::new(BUCKETS_KEY, store);
        if let Some(bucket) = config_store.get((*id).into()) {
            let ptrs: Self = bincode2::deserialize(&bucket)
                .map_err(|_| StdError::generic_err("Error deserializing bucket"))?;
            return Ok(ptrs);
        }

        Ok(Self {
            locations: Default::default(),
        })
    }

    pub fn insert_data_point(&mut self, geotime: GeoLocationTime) {
        let entry = self.locations.entry(geotime.geohash.clone()).or_default();
        entry.0.push(geotime.timestamp_ms);
    }

    fn _does_time_overlap(&self, ghash: &String, time: u64, period: u64) -> bool {
        if let Some(times) = self.locations.get(ghash) {
            // if we have data points for this location, check if the time overlaps, as well
            for t in &times.0 {
                if &time >= t && time <= t + period {
                    // if match, no need to look any further
                    return true;
                }
            }
        }
        return false;
    }

    pub fn match_pos(&self, ghash: &String, time: u64, period: u64) -> StdResult<bool> {
        // test our initial data point
        if self._does_time_overlap(ghash, time, period) {
            return Ok(true);
        }

        // find all geohash neighbors - possible optimizations:
        //     use integer geohashes
        //     a more optimized geohash curve
        //     for even more accuracy haversine distance can be used, but that requires storing
        // coordinates as well
        let positions = neighbors(ghash)?;

        // test all the neighbors of our geohash (since overlap may also be on the limits of the hash)
        for pos in positions {
            if self._does_time_overlap(&pos, time, period) {
                return Ok(true);
            }
        }
        return Ok(false);
    }
}

/// Load all our buckets at once for convenience when we know we will most likely need all of
/// them
pub fn load_all_buckets<S: Storage>(store: &S) -> StdResult<HashMap<BucketName, DailyBucket>> {
    let mut map = HashMap::<BucketName, DailyBucket>::default();
    for name in BucketName::iterator() {
        map.insert(name.clone(), DailyBucket::load(store, name)?);
    }

    Ok(map)
}

/// Initialize our buckets, according to a specific start time (time since epoch in milliseconds),
/// which will be the earliest allowed timestamp for data in our buckets.
/// The last allowed timestamp will be 14 * `ONE_DAY`
pub fn initialize_buckets<S: Storage>(store: &mut S, start_time: u64) -> StdResult<()> {
    let mut cur_time = start_time;
    let mut pointers = Pointers::default();
    for name in BucketName::iterator() {
        let new_pointer = Pointer {
            start_time: cur_time,
            end_time: cur_time + ONE_DAY,
            bucket: name.clone(),
        };
        cur_time = cur_time + ONE_DAY + 1;

        pointers.insert(new_pointer);
    }
    pointers.store(store)
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Locations(pub Vec<GeoLocationTime>);
//
// impl Default for Locations {
//     fn default() -> Self {
//         let this: Vec<GeoLocationTime> = vec![];
//
//         return Self { 0: this };
//     }
// }
