use bincode2;
use cosmwasm_std::{ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use radix_trie::{SubTrie, Trie, TrieCommon};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static TRIE_KEY: &[u8] = b"mytrie";
pub static TRIE_ID: &[u8] = b"mytrie";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MyTrie(pub Trie<String, u32>);

impl MyTrie {
    pub fn store<S: Storage>(&self, store: &mut S) -> StdResult<()> {
        let mut config_store = PrefixedStorage::new(TRIE_KEY, store);
        let as_bytes =
            bincode2::serialize(&self).map_err(|_| StdError::generic_err("Error packing trie"))?;

        config_store.set(TRIE_ID, &as_bytes);

        Ok(())
    }

    pub fn load<S: Storage>(store: &S) -> StdResult<Self> {
        let config_store = ReadonlyPrefixedStorage::new(TRIE_KEY, store);
        if let Some(trie) = config_store.get(TRIE_ID) {
            let ptrs: Self = bincode2::deserialize(&trie)
                .map_err(|_| StdError::generic_err("Error deserializing trie"))?;
            return Ok(ptrs);
        }

        Ok(Self::default())
    }

    pub fn remove(&mut self, key: &String) {
        let mut kill: bool = false;
        if let Some(val) = self.0.get_mut(key) {
            if val == 1 {
                kill = true;
            } else {
                *val -= 1
            }
        }
        if kill {
            self.0.remove(key)
        }
    }

    pub fn find_most_common(&self, depth: usize, mut hmap: &mut HashMap<String, u32>) {
        for sub in self.0.children() {
            Self::recursive_find_most_common(sub, depth, &mut hmap);
        }
    }

    fn recursive_find_most_common(
        t: SubTrie<String, u32>,
        depth: usize,
        mut hmap: &mut HashMap<String, u32>,
    ) {
        if let Some(key) = t.key() {
            if key.len() >= depth {
                if let Some(res) = hmap.get_mut(&key[..depth]) {
                    *res += t.get(key).unwrap().unwrap();
                } else {
                    hmap.insert((&key[..depth]).to_string(), *t.get(key).unwrap().unwrap());
                }
            }
            println!("{}, len: {}", key, key.len())
        }

        for sub in t.children() {
            Self::recursive_find_most_common(sub, depth, &mut hmap);
        }
    }
}
