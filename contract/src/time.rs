use crate::bucket::DailyBucket;
use crate::hotspotmap::HotspotMap;
use crate::msg::QueryAnswer;
use crate::pointer::{Pointer, Pointers, ONE_DAY};
use cosmwasm_std::{
    to_binary, Api, Env, Extern, HandleResponse, Querier, QueryResult, StdResult, Storage,
};

pub fn query_dates<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let pointers = Pointers::load(&deps.storage)?;

    let to = pointers.first().unwrap().end_time;
    let from = pointers.last().unwrap().start_time;

    return to_binary(&QueryAnswer::DateRange { from, to });
}

pub fn new_day<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    let mut pointers = Pointers::load(&deps.storage)?;

    let old_day = pointers.pop().unwrap();
    let old_bucket = DailyBucket::load(&deps.storage, &old_day.bucket)?;

    let new_day = Pointer {
        start_time: pointers.first().unwrap().end_time,
        end_time: pointers.first().unwrap().end_time + ONE_DAY,
        bucket: old_day.bucket,
    };

    let bucket = DailyBucket::default();
    bucket.store(&mut deps.storage, &old_day.bucket)?;
    pointers.insert(new_day);

    let mut hotspots = HotspotMap::load(&deps.storage)?;

    // might be better to create a trie per day, and aggregate it instead of doing it like this?
    // either way this only happens once per day, so might be acceptable to take a little more time
    // but still optimize for fast query
    for (loc, _) in old_bucket.locations.iter() {
        hotspots.remove_data_point(loc)
    }

    Ok(HandleResponse::default())
}
