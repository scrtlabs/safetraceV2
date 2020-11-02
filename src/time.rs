use crate::bucket::{Bucket, Pointer, Pointers, ONE_DAY};
use crate::msg::QueryAnswer;
use crate::trie::RecursiveTrie;
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
    let old_bucket = Bucket::load(&deps.storage, &old_day.bucket)?;

    let new_day = Pointer {
        start_time: pointers.first().unwrap().end_time,
        end_time: pointers.first().unwrap().end_time + ONE_DAY,
        bucket: old_day.bucket,
    };

    let bucket = Bucket::default();
    bucket.store(&mut deps.storage, &old_day.bucket)?;
    pointers.insert(new_day);

    let mut trie = RecursiveTrie::load(&deps.storage)?;

    // might be better to create a trie per day, and aggregate it instead of doing it like this?
    // either way this only happens once per day, so might be acceptable to take a little more time
    // but still optimize for fast query
    for (_, elem) in old_bucket.locations.iter() {
        for loc in elem.0.iter() {
            trie.remove(&loc.hash()?);
        }
    }

    Ok(HandleResponse::default())
}
