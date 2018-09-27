use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Bytes {
    pub data: Vec<u8>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SmallBytes {
    pub data: Vec<u8>
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
}

impl SmallBytes {
    pub fn from_vec(vec: Vec<u8>) -> SmallBytes {
        SmallBytes { data: vec }
    }
}