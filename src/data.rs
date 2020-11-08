use std::cmp::Ordering;
use std::convert::TryInto;

use cosmwasm_std::{
    to_binary, Api, Env, Extern, HandleResponse, Querier, QueryResult, StdResult, Storage,
};

use crate::bucket::{load_all_buckets, BucketName, DailyBucket};
use crate::geohash::GeoLocationTime;
use crate::hotspotmap::{HotSpots, HotspotMap};
use crate::msg::{GoogleLocation, GoogleTakeoutHistory, HotSpot, QueryAnswer};
use crate::pointer::Pointers;
use std::collections::HashMap;

pub const OVERLAP_TIME: u64 = 1000 * 60 * 5;

pub fn import_location_data<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data_points: GoogleTakeoutHistory,
) -> StdResult<HandleResponse> {
    // Generally speaking handles are pretty long - this should be acceptable, since they are
    // done once (per day), and there is a lot of processing done at this stage to ensure query
    // times are as low as possible, since that is what the responsiveness of a system which uses
    // this data would be based on
    let pointers = Pointers::load(&deps.storage)?;

    // Load all the buckets already, since we assume we will be inserting a large amount of data
    // (can be optimized to lazy-load each bucket)
    let mut buckets = load_all_buckets(&deps.storage)?;

    // this structure stores geohashes with less accuracy, as well as the amount of times that
    // a specific hash has been seen. The data structure also maintains a list of the top most
    // inserted keys. That way we end up with the top hot spots automatically at the end of the
    // insertion.
    let mut hotspot_map = HotspotMap::load(&deps.storage)?;

    for dp in data_points.locations {
        // If the data point is dated after or before our two week window, just ignore it.
        // Most of these should be handled in pre-processing
        if let Some(bucket) = pointers.find_bucket(dp.timestampMs.u128() as u64) {
            // convert to our internal structure (geohash + time)
            let geopt: GeoLocationTime = dp.try_into()?;

            // insert data into our hot spot tracker - we only need the hash for this,
            // not the timepoint
            hotspot_map.insert_data_point(geopt.geohash.clone());

            // insert data into time-space tracker.
            buckets.get_mut(&bucket).unwrap().insert_data_point(geopt);
        }
    }

    // we extract the top hotspots now, so we can directly query it
    let hotspot_cache = HotSpots(hotspot_map.get_top_hotspots());

    // store all buckets
    for (name, b) in buckets {
        b.store(&mut deps.storage, &name)?;
    }

    hotspot_map.store(&mut deps.storage)?;

    hotspot_cache.store(&mut deps.storage)?;

    // no need to return any special response
    Ok(HandleResponse::default())
}

pub fn match_data_point<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    data_points: Vec<GoogleLocation>,
) -> QueryResult {
    let pointers = Pointers::load(&deps.storage)?;
    let mut geo_overlap: Vec<GeoLocationTime> = Vec::default();

    // loading each bucket at 4 mil data points takes about ~4 seconds, so we cache results to
    // not read from disk and decrypt twice
    let mut bucket_cache: HashMap<BucketName, DailyBucket> = HashMap::default();

    for dp in data_points {
        if let Some(bucket_name) = pointers.find_bucket(dp.timestampMs.u128() as u64) {
            if !bucket_cache.contains_key(&bucket_name) {
                let bucket = DailyBucket::load(&deps.storage, &bucket_name)?;
                bucket_cache.insert(bucket_name.clone(), bucket);
            }

            let geoloc: GeoLocationTime = dp.try_into()?;
            // matches according to geohash and time
            if bucket_cache.get(&bucket_name).unwrap().match_pos(
                &geoloc.geohash,
                geoloc.timestamp_ms,
                OVERLAP_TIME,
            )? {
                geo_overlap.push(geoloc);
            }
        }
    }
    to_binary(&QueryAnswer::Overlap {
        data_points: geo_overlap,
    })
}

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

// pub const DISTANCE: f64 = 10.0; // in meters
// pub const EARTH_RADIUS: f64 = 6371000.0; // in meters
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
