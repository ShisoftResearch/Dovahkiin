use crate::types::SharedMap;

use super::map::Map;
use super::{super::*, shared_map::key_hash};
use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;
use std::slice::Iter;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct OwnedMap {
    pub map: HashMap<u64, OwnedValue>,
    pub fields: Vec<String>,
}

impl Map for OwnedMap {

    type Value = OwnedValue;

    fn new() -> Self {
        Self {
            map: HashMap::new(),
            fields: Vec::new(),
        }
    }
    fn from_hash_map(map: HashMap<String, Self::Value>) -> Self {
        let mut target_map = HashMap::new();
        let fields = map.keys().cloned().collect();
        for (key, value) in map {
            target_map.insert(key_hash(&key), value);
        }
        Self {
            map: target_map,
            fields,
        }
    }
    fn insert<'a>(&mut self, key: &'a str, value: Self::Value) -> Option<Self::Value> {
        self.fields.push(key.to_string());
        self.insert_key_id(key_hash(key), value)
    }
    fn insert_key_id(&mut self, key: u64, value: Self::Value) -> Option<Self::Value> {
        self.map.insert(key, value)
    }

    fn get_by_key_id(&self, key: u64) -> &Self::Value {
        self.map.get(&key).unwrap_or(&NULL_OWNED_VALUE)
    }
    fn get_mut_by_key_id(&mut self, key: u64) -> &mut Self::Value {
        self.map.entry(key).or_insert(Self::Value::Null)
    }
    fn get<'a>(&self, key: &'a str) -> &Self::Value {
        self.get_by_key_id(key_hash(key))
    }
    fn get_mut<'a>(&mut self, key: &'a str) -> &mut Self::Value {
        self.get_mut_by_key_id(key_hash(key))
    }
    fn get_in_by_ids<'a, I: Iterator<Item = &'a u64> + ExactSizeIterator>(
        &self,
        mut key_ids: I,
    ) -> &Self::Value {
        let current_key = key_ids.next().cloned();
        if let Some(key) = current_key {
            let value = self.get_by_key_id(key);
            if key_ids.is_empty() {
                return value;
            } else {
                match value {
                    &Self::Value::Map(ref map) => return map.get_in_by_ids(key_ids),
                    _ => {}
                }
            }
        }
        return &NULL_OWNED_VALUE;
    }
    fn get_in(&self, keys: &[&'static str]) -> &Self::Value {
        self.get_in_by_ids(Self::strs_to_ids(keys).iter())
    }
    fn get_in_mut_by_key_ids(&mut self, mut keys_ids: Iter<u64>) -> Option<&mut Self::Value> {
        let current_key = keys_ids.next().cloned();
        if let Some(key) = current_key {
            let value = self.get_mut_by_key_id(key);
            match value {
                &mut Self::Value::Null => return None,
                _ => {
                    if keys_ids.is_empty() {
                        return Some(value);
                    } else {
                        match value {
                            &mut Self::Value::Map(ref mut map) => {
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
    fn get_in_mut(&mut self, keys: &[&'static str]) -> Option<&mut Self::Value> {
        self.get_in_mut_by_key_ids(Self::strs_to_ids(keys).iter())
    }
    fn update_in_by_key_ids<U>(&mut self, keys: Iter<u64>, update: U) -> Option<()>
    where
        U: FnOnce(&mut Self::Value),
    {
        let value = self.get_in_mut_by_key_ids(keys);
        if let Some(value) = value {
            update(value);
            Some(())
        } else {
            None
        }
    }
    fn update_in<U>(&mut self, keys: &[&'static str], update: U) -> Option<()>
    where
        U: FnOnce(&mut Self::Value),
    {
        self.update_in_by_key_ids(Self::strs_to_ids(keys).iter(), update)
    }
    fn set_in_by_key_ids(&mut self, keys: Iter<u64>, value: Self::Value) -> Option<()> {
        let val = self.get_in_mut_by_key_ids(keys);
        if let Some(val) = val {
            *val = value;
            Some(())
        } else {
            None
        }
    }
    fn set_in(&mut self, keys: &[&'static str], value: Self::Value) -> Option<()> {
        self.set_in_by_key_ids(Self::strs_to_ids(keys).iter(), value)
    }
    fn into_string_map(self) -> HashMap<String, Self::Value> {
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
    fn len(&self) -> usize {
        self.map.len()
    }
    fn shared<'a>(&'a self) -> SharedMap<'a> {
        SharedMap {
            fields: self.fields.clone(),
            map: self.map.iter().map(|(k, v)| (*k, v.shared())).collect(),
        }
    }

    fn owned(&self) -> OwnedMap {
        self.clone()
    }
}

impl OwnedMap {
    pub fn insert_value<'a, V>(&mut self, key: &'a str, value: V) -> Option<OwnedValue>
    where
        V: ToValue,
    {
        self.insert(key, value.value())
    }
    pub fn insert_key_id_value<V>(&mut self, key: u64, value: V) -> Option<OwnedValue>
    where
        V: ToValue,
    {
        self.insert_key_id(key, value.value())
    }
}

impl fmt::Debug for OwnedMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( ")?;
        for name in &self.fields {
            let id = key_hash(name);
            write!(f, "{}: {:?} ", name, self.map[&id])?;
        }
        write!(f, ") ")
    }
}

impl PartialOrd for OwnedMap {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        unreachable!("Cannot compare maps")
    }
}
