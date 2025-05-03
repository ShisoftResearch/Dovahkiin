use crate::types::{key_hash, OwnedMap, SharedMap, Value};
use ahash::HashMap;
use serde::Serialize;
use smallvec::SmallVec;
use std::{collections::BTreeMap, slice::Iter};
use std::borrow::Borrow;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

pub trait Map {
    type Value: Value;
    fn new() -> Self;
    fn from_pairs<P: IntoIterator<Item = (String, Self::Value)>>(map: P) -> Self;
    fn insert<'a>(&mut self, key: &'a str, value: Self::Value) -> Option<Self::Value>;
    fn insert_key_id(&mut self, key: u64, value: Self::Value) -> Option<Self::Value>;
    fn get_by_key_id(&self, key: u64) -> &Self::Value;
    fn get_mut_by_key_id(&mut self, key: u64) -> &mut Self::Value;
    fn get<'a>(&self, key: &'a str) -> &Self::Value;
    fn get_mut<'a>(&mut self, key: &'a str) -> &mut Self::Value;
    fn strs_to_ids<'a>(keys: &[&'a str]) -> Vec<u64> {
        keys.iter().map(|str| key_hash(str)).collect()
    }
    fn get_in_by_ids<'a, I: Iterator<Item = &'a u64> + ExactSizeIterator>(
        &self,
        key_ids: I,
    ) -> &Self::Value;
    fn get_in(&self, keys: &[&'static str]) -> &Self::Value;
    fn get_in_mut_by_key_ids(&mut self, keys_ids: Iter<u64>) -> Option<&mut Self::Value>;
    fn get_in_mut(&mut self, keys: &[&'static str]) -> Option<&mut Self::Value>;
    fn update_in_by_key_ids<U>(&mut self, keys: Iter<u64>, update: U) -> Option<()>
    where
        U: FnOnce(&mut Self::Value);
    fn update_in<U>(&mut self, keys: &[&'static str], update: U) -> Option<()>
    where
        U: FnOnce(&mut Self::Value);
    fn set_in_by_key_ids(&mut self, keys: Iter<u64>, value: Self::Value) -> Option<()>;
    fn set_in(&mut self, keys: &[&'static str], value: Self::Value) -> Option<()>;
    fn into_string_map(self) -> HashMap<String, Self::Value>;
    fn len(&self) -> usize;
    fn shared<'a>(&'a self) -> SharedMap<'a>;
    fn owned(&self) -> OwnedMap;
}

const SMALL_MAP_CAPACITY: usize = 16;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct SmallMap<K, V> {
    pairs: Vec<(K, V)>,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum GenericMap<K: Ord, V> {
    Small(SmallMap<K, V>),
    Large(BTreeMap<K, V>),
}

impl<K: Ord, V> GenericMap<K, V> {
    pub fn new() -> Self {
        GenericMap::Small(SmallMap::new())
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.check_upgrade();
        match self {
            GenericMap::Small(small) => small.insert(key, value),
            GenericMap::Large(large) => large.insert(key, value),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self {
            GenericMap::Small(small) => small.get(key),
            GenericMap::Large(large) => large.get(key),
        }
    }
    
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self {
            GenericMap::Small(small) => small.get_mut(key),
            GenericMap::Large(large) => large.get_mut(key),
        }
    }
    
    pub fn len(&self) -> usize {
        match self {
            GenericMap::Small(small) => small.len(),
            GenericMap::Large(large) => large.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            GenericMap::Small(small) => small.is_empty(),
            GenericMap::Large(large) => large.is_empty(),
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self {
            GenericMap::Small(small) => small.remove(key),
            GenericMap::Large(large) => large.remove(key),
        }
    }
    
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a K, &'a V)> + 'a> {
        match self {
            GenericMap::Small(small) => Box::new(small.pairs.iter().map(|(k, v)| (k, v))),
            GenericMap::Large(large) => Box::new(large.iter()),
        }
    }

    pub fn into_iter(self) -> Box<dyn Iterator<Item = (K, V)>> 
    where
        K: 'static,
        V: 'static,
    {
        match self {
            GenericMap::Small(small) => Box::new(small.pairs.into_iter()),
            GenericMap::Large(large) => Box::new(large.into_iter()),
        }
    }

    fn check_upgrade(&mut self) {
        match self {
            GenericMap::Small(small) => {
                if small.len() + 1 >= SMALL_MAP_CAPACITY {
                    let mut large = BTreeMap::new();
                    for (k, v) in small.pairs.drain(..small.len()) {
                        large.insert(k, v);
                    }
                    *self = GenericMap::Large(large);
                }
            }
            GenericMap::Large(_) => {}
        }
    }

    // Gets a mutable reference to the value corresponding to the key or inserts a default value
    pub fn get_or_insert(&mut self, key: K, default: V) -> &mut V 
    where K: Clone
    {
        self.check_upgrade();
        let has_key = match self {
            GenericMap::Small(small) => small.get(&key).is_some(),
            GenericMap::Large(large) => large.contains_key(&key),
        };
        
        if !has_key {
            self.insert(key.clone(), default);
        }
        
        match self {
            GenericMap::Small(small) => {
                for (k, v) in &mut small.pairs {
                    if k == &key {
                        return v;
                    }
                }
                unreachable!("Key not found after insertion")
            }
            GenericMap::Large(large) => large.get_mut(&key).unwrap(),
        }
    }
    
    // Gets a mutable reference to the value corresponding to the key or 
    // inserts a value computed from the default function
    pub fn get_or_insert_with<F: FnOnce() -> V>(&mut self, key: K, default: F) -> &mut V 
    where K: Clone
    {
        self.check_upgrade();
        let has_key = match self {
            GenericMap::Small(small) => small.get(&key).is_some(),
            GenericMap::Large(large) => large.contains_key(&key),
        };
        
        if !has_key {
            let value = default();
            self.insert(key.clone(), value);
        }
        
        match self {
            GenericMap::Small(small) => {
                for (k, v) in &mut small.pairs {
                    if k == &key {
                        return v;
                    }
                }
                unreachable!("Key not found after insertion")
            }
            GenericMap::Large(large) => large.get_mut(&key).unwrap(),
        }
    }
}

impl<K: PartialEq, V> SmallMap<K, V> {
    pub fn new() -> Self {
        SmallMap {
            pairs: Vec::with_capacity(SMALL_MAP_CAPACITY),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        for (k, v) in self.pairs.iter_mut() {
            if k == &key {
                return Some(std::mem::replace(v, value));
            }
        }
        self.pairs.push((key, value));
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for (k, v) in self.pairs.iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for (k, v) in self.pairs.iter_mut() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        for (i, (k, _v)) in self.pairs.iter().enumerate() {
            if k == key {
                return Some(self.pairs.remove(i).1);
            }
        }
        None
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for GenericMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = GenericMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

impl<K: Ord, V> Index<&K> for GenericMap<K, V> {
    type Output = V;
    
    fn index(&self, key: &K) -> &Self::Output {
        match self.get(key) {
            Some(val) => val,
            None => panic!("Key not found in GenericMap"),
        }
    }
}

impl<K: Ord, V> IndexMut<&K> for GenericMap<K, V> {
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        match self.get_mut(key) {
            Some(val) => val,
            None => panic!("Key not found in GenericMap"),
        }
    }
}

