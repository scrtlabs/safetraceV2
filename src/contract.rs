use cosmwasm_std::{
    to_binary, Api, Env, Extern, HandleResponse, InitResponse, Querier, QueryResult, StdError,
    StdResult, Storage,
};

use crate::bucket::{initialize_buckets, load_all_buckets, Bucket, Pointer, Pointers, ONE_DAY};
use crate::data::{add_data_points, add_google_data, cluster, match_data_point};
use crate::msg::QueryAnswer::DateRange;
use crate::msg::{HandleMsg, HotSpot, InitMsg, QueryAnswer, QueryMsg};
use crate::trie::MyTrie;
use geohash::{encode, Coordinate};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    initialize_buckets(&mut deps.storage, msg.start_time)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddDataPoints { data_points } => add_data_points(deps, env, data_points),
        HandleMsg::NewDay {} => new_day(deps, env),
        HandleMsg::ImportGoogleLocations { data } => add_google_data(deps, env, data),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::MatchDataPoint { data_point } => match_data_point(deps, data_point),
        QueryMsg::HotSpot {} => hotspots(deps),
        QueryMsg::TimeRange {} => query_dates(deps),
    }
}

pub fn query_dates<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let mut pointers = Pointers::load(&deps.storage)?;

    let to = pointers.first().unwrap().end_time;
    let from = pointers.last().unwrap().start_time;

    return to_binary(&QueryAnswer::DateRange { from, to });
}

pub fn hotspots<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let trie = MyTrie::load(&deps.storage)?;

    let res: Vec<HotSpot> = cluster(&trie, 7, 10)
        .into_iter()
        .map(|kv| HotSpot {
            geo_location: kv.0,
            power: kv.1,
        })
        .collect();

    return to_binary(&QueryAnswer::HotSpotResponse { hot_spots: res });
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

    let mut bucket = Bucket::default();
    bucket.store(&mut deps.storage, &old_day.bucket);
    pointers.insert(new_day);

    let mut trie = MyTrie::load(&deps.storage)?;

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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::time::{Duration, Instant};

    use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{Coin, Env, Extern, HumanAddr, InitResponse, MemoryStorage, StdResult};
    use serde::{Deserialize, Serialize};
    use serde_json;

    use crate::contract::init;
    use crate::data::add_google_data;
    use crate::msg::HandleMsg::ImportGoogleLocations;
    use crate::msg::InitMsg;

    pub const MOCK_CONTRACT_ADDR: &str = "cosmos2contract";

    pub fn init_deps(
        canonical_length: usize,
        contract_balance: &[Coin],
    ) -> Extern<MockStorage, MockApi, MockQuerier> {
        let contract_addr = HumanAddr::from(MOCK_CONTRACT_ADDR);
        Extern {
            storage: MemoryStorage::default(),
            api: MockApi::new(canonical_length),
            querier: MockQuerier::new(&[(&contract_addr, contract_balance)]),
        }
    }

    fn init_helper() -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
        Env,
    ) {
        let mut deps = init_deps(20, &[]);
        let env = mock_env("instantiator", &[]);

        let init_msg = InitMsg {
            start_time: 1600129528955,
        };

        (init(&mut deps, env.clone(), init_msg), deps, env)
    }

    fn load_google_data() -> Vec<u8> {
        let mut cert = vec![];
        let mut f = File::open("tests/data/datamsg2.json").unwrap();
        f.read_to_end(&mut cert).unwrap();

        cert
    }

    #[test]
    pub fn test_add_google_data() {
        let data_msg: crate::msg::HandleMsg = serde_json::from_slice(&load_google_data()).unwrap();

        let (res, mut deps, env) = init_helper();
        let now = Instant::now();
        crate::contract::handle(&mut deps, env, data_msg);
        println!("elapsed: {}", now.elapsed().as_millis());
    }
}
