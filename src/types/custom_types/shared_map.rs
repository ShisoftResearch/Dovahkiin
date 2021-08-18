use super::super::*;
use bifrost_hasher::hash_str;
use std::collections::HashMap;
use std::iter::Iterator;
use std::slice::Iter;

type Value<'a> = SharedValue<'a>;

#[derive(Debug, PartialEq, Clone)]
pub struct SharedMap<'v> {
    pub map: HashMap<u64, Value<'v>>,
    pub fields: Vec<String>,
}
impl <'v> SharedMap<'v> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            fields: Vec::new(),
        }
    }
    pub fn from_hash_map(map: HashMap<String, Value<'v>>) -> Self {
        let mut target_map: HashMap<_, SharedValue<'v>> = HashMap::new();
        let fields = map.keys().cloned().collect();
        for (key, value) in map {
            target_map.insert(key_hash(&key), value);
        }
        Self {
            map: target_map,
            fields,
        }
    }
    pub fn owned(&self) -> OwnedMap {
        OwnedMap {
            map: self.map.iter().map(|(k, v)| (*k, v.owned())).collect(),
            fields: self.fields.clone(),
        }
    }
    pub fn insert<'a>(&mut self, key: &'a str, value: Value<'v>) -> Option<Value<'v>> {
        self.fields.push(key.to_string());
        self.insert_key_id(key_hash(key), value)
    }
    pub fn insert_key_id(&mut self, key: u64, value: Value<'v>) -> Option<Value<'v>> {
        self.map.insert(key, value)
    }
    pub fn get_by_key_id(&self, key: u64) -> &Value<'v> {
        self.map.get(&key).unwrap_or(&Value::Null)
    }
    pub fn get_mut_by_key_id(&mut self, key: u64) -> &mut Value<'v> {
        self.map.entry(key).or_insert(Value::Null)
    }
    pub fn get<'a>(&self, key: &'a str) -> &Value<'v> {
        self.get_by_key_id(key_hash(key))
    }
    pub fn get_mut<'a>(&mut self, key: &'a str) -> &mut Value<'v> {
        self.get_mut_by_key_id(key_hash(key))
    }
    pub fn strs_to_ids<'a>(keys: &[&'a str]) -> Vec<u64> {
        keys.iter().map(|str| key_hash(str)).collect()
    }
    pub fn get_in_by_ids<'a, I: Iterator<Item = &'a u64> + ExactSizeIterator>(
        &self,
        mut key_ids: I,
    ) -> &Value<'v> {
        let current_key = key_ids.next().cloned();
        if let Some(key) = current_key {
            let value = self.get_by_key_id(key);
            if key_ids.is_empty() {
                return value;
            } else {
                match value {
                    &Value::Map(ref map) => return map.get_in_by_ids(key_ids),
                    _ => {}
                }
            }
        }
        return &Value::Null;
    }
    pub fn get_in(&self, keys: &[&'static str]) -> &Value<'v> {
        self.get_in_by_ids(Self::strs_to_ids(keys).iter())
    }
    pub fn get_in_mut_by_key_ids(&mut self, mut keys_ids: Iter<u64>) -> Option<&mut Value<'v>> {
        let current_key = keys_ids.next().cloned();
        if let Some(key) = current_key {
            let value = self.get_mut_by_key_id(key);
            match value {
                &mut Value::Null => return None,
                _ => {
                    if keys_ids.is_empty() {
                        return Some(value);
                    } else {
                        match value {
                            &mut Value::Map(ref mut map) => {
                                return map.get_in_mut_by_key_ids(keys_ids)
                            }
                            _ => return None,
                        }
                    }
                }
            }
        } else {
            return None;
        }
    }
    pub fn get_in_mut(&mut self, keys: &[&'static str]) -> Option<&mut Value<'v>> {
        self.get_in_mut_by_key_ids(Self::strs_to_ids(keys).iter())
    }
    pub fn update_in_by_key_ids<U>(&mut self, keys: Iter<u64>, update: U) -> Option<()>
    where
        U: FnOnce(&mut Value),
    {
        let value = self.get_in_mut_by_key_ids(keys);
        if let Some(value) = value {
            update(value);
            Some(())
        } else {
            None
        }
    }
    pub fn update_in<U>(&mut self, keys: &[&'static str], update: U) -> Option<()>
    where
        U: FnOnce(&mut Value),
    {
        self.update_in_by_key_ids(Self::strs_to_ids(keys).iter(), update)
    }
    pub fn set_in_by_key_ids(&mut self, keys: Iter<u64>, value: Value<'v>) -> Option<()> {
        let val = self.get_in_mut_by_key_ids(keys);
        if let Some(val) = val {
            *val = value;
            Some(())
        } else {
            None
        }
    }
    pub fn set_in(&mut self, keys: &[&'static str], value: Value<'v>) -> Option<()> {
        self.set_in_by_key_ids(Self::strs_to_ids(keys).iter(), value)
    }
    pub fn into_string_map(self) -> HashMap<String, Value<'v>> {
        let mut id_map: HashMap<u64, String> = self
            .fields
            .into_iter()
            .map(|field| (key_hash(&field), field))
            .collect();
        self.map
            .into_iter()
            .map(|(fid, value)| (id_map.remove(&fid), value))
            .filter(|&(ref field, _)| field.is_some())
            .map(|(field, value)| (field.unwrap(), value))
            .collect()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

pub fn key_hash<'a>(key: &'a str) -> u64 {
    hash_str(key)
}

pub fn key_hashes(keys: &Vec<String>) -> Vec<u64> {
    keys.iter().map(|str| hash_str(str)).collect()
}
