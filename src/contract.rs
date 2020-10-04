use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};

use crate::bucket::initialize_buckets;
use crate::data::{add_data_points, add_google_data, match_data_point};
use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    initialize_buckets(&mut deps.storage, msg.start_time);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddDataPoints { data_points } => add_data_points(deps, env, data_points),
        HandleMsg::ImportGoogleLocations { data } => add_google_data(deps, env, data),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::MatchDataPoint { data_point } => match_data_point(deps, data_point),
        QueryMsg::HotSpot {} => Ok(Binary::default()),
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::init;
    use crate::data::add_google_data;
    use crate::msg::HandleMsg::ImportGoogleLocations;
    use crate::msg::InitMsg;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{Coin, Env, Extern, HumanAddr, InitResponse, MemoryStorage, StdResult};
    use serde::{Deserialize, Serialize};
    use serde_json;
    use std::fs::File;
    use std::io::Read;
    use std::time::{Duration, Instant};
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
