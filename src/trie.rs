use bincode2;
use cosmwasm_std::{ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use radix_trie::{SubTrie, Trie, TrieCommon};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static TRIE_KEY: &[u8] = b"mytrie";
pub static TRIE_ID: &[u8] = b"mytrie";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Leaf {
    pub sum_children: u32,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RecursiveTrie(pub Trie<String, Leaf>);

/// A radix-trie that saves the sum of all child values as a part of the node data. The reason for this is to
/// make queries just a tiny bit faster (avoiding those last expensive recursion steps)
impl RecursiveTrie {
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

    pub fn insert(&mut self, key: String, val: u32) {
        self.0.map_with_default(
            key.clone(),
            |leaf| leaf.value += val,
            Leaf {
                sum_children: 0,
                value: val,
            },
        );
        self.update_ancestor(&key, val);
    }

    pub fn update_ancestor(&mut self, key: &String, val: u32) {
        let mut more = false;
        let mut next_key: String = String::default();

        if let Some(res) = self.0.get_ancestor(key) {
            //println!("Started with key: {}, got ancestor with key: {}", key, res.key().unwrap());
            if res.key().unwrap() != key {
                next_key = res.key().unwrap().clone();
                more = true;
            }
        }
        if more {
            if let Some(mut a) = self.0.subtrie_mut(&next_key) {
                a.value_mut().unwrap().sum_children += val;
            }
        }
        if more {
            self.update_ancestor(&next_key, val);
        }
    }

    pub fn remove(&mut self, key: &String) {
        let mut kill: bool = false;
        if let Some(val) = self.0.get_mut(key) {
            if (*val).value == 1 {
                kill = true;
            } else {
                (*val).value -= 1
            }
        }
        if kill {
            self.0.remove(key);
        }
    }

    pub fn find_most_common(&self, depth: usize, mut hmap: &mut HashMap<String, u32>) {
        for sub in self.0.children() {
            Self::recursive_find_most_common(sub, depth, &mut hmap);
        }
    }

    fn recursive_find_most_common(
        t: SubTrie<String, Leaf>,
        depth: usize,
        mut hmap: &mut HashMap<String, u32>,
    ) {
        if let Some(key) = t.key() {
            if key.len() >= depth {
                let leaf = t.get(key).unwrap().unwrap();
                let geohash = (&key[..depth]).to_string();
                if hmap.contains_key(&geohash) {
                    *(hmap.get_mut(&geohash).unwrap()) += leaf.sum_children + leaf.value;
                } else {
                    hmap.insert(geohash, leaf.sum_children + leaf.value);
                }
            // if let Some(res) = hmap.get_mut(&key[..depth]) {
            //     *res += t.get(key).unwrap().unwrap().value;
            // } else {
            //     hmap.insert(
            //         (&key[..depth]).to_string(),
            //         (*t.get(key).unwrap().unwrap()).value,
            //     );
            // }
            } else {
                for sub in t.children() {
                    Self::recursive_find_most_common(sub, depth, &mut hmap);
                }
            }
        } else {
            for sub in t.children() {
                Self::recursive_find_most_common(sub, depth, &mut hmap);
            }
        }
    }
}
