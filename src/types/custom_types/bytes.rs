use std::ops::Deref;
use std::ops::DerefMut;

use bifrost::utils::serde::{deserialize, serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Bytes {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SmallBytes {
    pub data: Vec<u8>,
}

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &'_ <Self as Deref>::Target {
        &self.data
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &'_ mut <Self as Deref>::Target {
        &mut self.data
    }
}

impl Deref for SmallBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &'_ <Self as Deref>::Target {
        &self.data
    }
}

impl DerefMut for SmallBytes {
    fn deref_mut(&mut self) -> &'_ mut <Self as Deref>::Target {
        &mut self.data
    }
}

impl Bytes {
    pub fn from_vec(vec: Vec<u8>) -> Bytes {
        Bytes { data: vec }
    }
    pub fn to<'a, T>(&'a self) -> T
    where
        T: serde::Deserialize<'a>,
    {
        deserialize(&self.data).unwrap()
    }
    pub fn from<T>(data: &T) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            data: serialize(data),
        }
    }
}

impl SmallBytes {
    pub fn from_vec(vec: Vec<u8>) -> SmallBytes {
        SmallBytes { data: vec }
    }
}

impl From<Vec<u8>> for SmallBytes {
    fn from(vec: Vec<u8>) -> Self {
        Self {
            data: vec
        }
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(vec: Vec<u8>) -> Self {
        Self {
            data: vec
        }
    }
}
