use core::option::Option::{None, Some};
use core::result::Result::Ok;

use bincode2;
use cosmwasm_std::{ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use serde::{Deserialize, Serialize};

use crate::bucket::BucketName;

pub const ONE_DAY: u64 = 1000 * 60 * 60 * 24;
pub static POINTERS_KEY: &[u8] = b"pointers";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pointer {
    pub start_time: u64,
    pub end_time: u64,
    pub bucket: BucketName,
}

/// `Pointers` is a structure that we used to identify our daily buckets,
///
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Pointers(pub Vec<Pointer>);

impl Pointers {
    pub fn store<S: Storage>(&self, store: &mut S) -> StdResult<()> {
        let mut config_store = PrefixedStorage::new(POINTERS_KEY, store);
        let as_bytes = bincode2::serialize(&self)
            .map_err(|_| StdError::generic_err("Error serializing pointers"))?;

        config_store.set(POINTERS_KEY, &as_bytes);

        Ok(())
    }

    pub fn load<S: Storage>(store: &S) -> StdResult<Self> {
        let config_store = ReadonlyPrefixedStorage::new(POINTERS_KEY, store);
        if let Some(temp) = config_store.get(POINTERS_KEY) {
            let ptrs: Self = bincode2::deserialize(&temp)
                .map_err(|_| StdError::generic_err("Error deserializing pointers"))?;
            return Ok(ptrs);
        }

        Ok(Self::default())
    }

    pub fn find_bucket(&self, time: u64) -> Option<BucketName> {
        for p in &self.0 {
            if time >= p.start_time && time <= p.end_time {
                return Some(p.bucket);
            }
        }
        None
    }

    pub fn sort(&mut self) {
        self.0
            .sort_unstable_by(|a, b| a.start_time.cmp(&b.start_time))
    }

    pub fn pop(&mut self) -> Option<Pointer> {
        self.0.pop()
    }

    pub fn insert(&mut self, ptr: Pointer) {
        self.0.push(ptr);
        self.sort();
    }

    pub fn first(&self) -> Option<&Pointer> {
        self.0.first()
    }

    pub fn last(&self) -> Option<&Pointer> {
        self.0.last()
    }
}
