use crate::bucket::{load_all_buckets, Bucket, GeoLocationTime, Pointers};
use crate::msg::{GoogleTakeoutHistory, QueryMsg};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, Querier, StdResult, Storage,
};

pub const DISTANCE: f64 = 10.0; // in meters
pub const EARTH_RADIUS: f64 = 6371000.0; // in meters
pub const OVERLAP_TIME: u64 = 1000 * 60 * 5;
pub fn add_data_points<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data_points: Vec<GeoLocationTime>,
) -> StdResult<HandleResponse> {
    // let pointers = Pointers::load(&deps.storage)?;
    //
    // for dp in data_points {
    //     if let Some(bucket) = pointers.find_bucket(dp.timestamp_ms) {
    //         let mut dis = Bucket::load(&deps.storage, &bucket)?;
    //
    //         dis.insert_data_point(dp);
    //
    //         dis.store(&mut deps.storage, &bucket);
    //     }
    // }

    Ok(HandleResponse::default())
}

pub fn add_google_data<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data_points: GoogleTakeoutHistory,
) -> StdResult<HandleResponse> {
    let pointers = Pointers::load(&deps.storage)?;

    let mut buckets = load_all_buckets(&deps.storage)?;

    for dp in data_points.locations {
        if let Some(bucket) = pointers.find_bucket(dp.timestampMs.u128() as u64) {
            //let mut dis = Bucket::load(&deps.storage, &bucket)?;

            buckets
                .get_mut(&bucket)
                .unwrap()
                .insert_data_point(dp.into());
        }
    }

    for (name, b) in buckets {
        b.store(&mut deps.storage, &name);
    }

    Ok(HandleResponse::default())
}

pub fn match_data_point<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    data_point: GeoLocationTime,
) -> StdResult<Binary> {
    let pointers = Pointers::load(&deps.storage)?;
    let mut geo_overlap: Vec<GeoLocationTime> = Vec::default();

    if let Some(bucket) = pointers.find_bucket(data_point.timestamp_ms) {
        let mut dis = Bucket::load(&deps.storage, &bucket)?;

        let time_overlap = dis.search(
            data_point.timestamp_ms,
            data_point.timestamp_ms + OVERLAP_TIME,
        );
        if time_overlap.len() > 0 {
            for point in time_overlap {
                if match_location(&point, &data_point) {
                    geo_overlap.push(point.clone());
                }
            }
        } else {
            return Ok(Binary::from(
                format!("No overlapping times found broheim").as_bytes(),
            ));
        }
        return if geo_overlap.len() > 0 {
            Ok(Binary::from(
                format!(
                    "Found lots of overlap. You might be sick dawg: {:?}",
                    geo_overlap[0]
                )
                .as_bytes(),
            ))
        } else {
            Ok(Binary::from(
                format!("No overlapping locations found, broseph",).as_bytes(),
            ))
        };
    }

    Ok(Binary::from(
        format!("Time is either too old, or too recent. Either way, you're all good bruv")
            .as_bytes(),
    ))
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
