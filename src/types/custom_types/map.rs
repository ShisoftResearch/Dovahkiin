use std::{collections::HashMap, slice::Iter};

use crate::types::{Value, key_hash, SharedMap, OwnedMap};

pub trait Map {
    type Value: Value;
    fn new() -> Self;
    fn from_hash_map(map: HashMap<String, Self::Value>) -> Self;
    fn insert<'a>(&mut self, key: &'a str, value: Self::Value) -> Option<Self::Value>;
    fn insert_key_id(&mut self, key: u64, value: Self::Value) -> Option<Self::Value>;
    fn get_by_key_id(&self, key: u64) -> &Self::Value;
    fn get_mut_by_key_id(&mut self, key: u64) -> &mut Self::Value;
    fn get<'a>(&self, key: &'a str) -> &Self::Value;
    fn get_mut<'a>(&mut self, key: &'a str) -> &mut Self::Value;
    fn strs_to_ids<'a>(keys: &[&'a str]) -> Vec<u64> {
        keys.iter().map(|str| key_hash(str)).collect()
    }
    fn get_in_by_ids<'a, I: Iterator<Item = &'a u64> + ExactSizeIterator>(&self, key_ids: I) -> &Self::Value;
    fn get_in(&self, keys: &[&'static str]) -> &Self::Value;
    fn get_in_mut_by_key_ids(&mut self, keys_ids: Iter<u64>) -> Option<&mut Self::Value>;
    fn get_in_mut(&mut self, keys: &[&'static str]) -> Option<&mut Self::Value>;
    fn update_in_by_key_ids<U>(&mut self, keys: Iter<u64>, update: U) -> Option<()> where U: FnOnce(&mut Self::Value);
    fn update_in<U>(&mut self, keys: &[&'static str], update: U) -> Option<()> where U: FnOnce(&mut Self::Value);
    fn set_in_by_key_ids(&mut self, keys: Iter<u64>, value: Self::Value) -> Option<()>;
    fn set_in(&mut self, keys: &[&'static str], value: Self::Value) -> Option<()>;
    fn into_string_map(self) -> HashMap<String, Self::Value>;
    fn len(&self) -> usize;
    fn shared<'a>(&'a self) -> SharedMap<'a>;
    fn owned(&self) -> OwnedMap;
}