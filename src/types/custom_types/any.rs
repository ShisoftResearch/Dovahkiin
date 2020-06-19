use bifrost::utils::serde::{deserialize, serialize};
use serde;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Any {
    pub data: Vec<u8>,
}
impl Any {
    pub fn to<'a, T>(&'a self) -> T
    where
        T: serde::Deserialize<'a>,
    {
        deserialize(&self.data).unwrap()
    }
    pub fn from<T>(data: &T) -> Any
    where
        T: serde::Serialize,
    {
        Any {
            data: serialize(data),
        }
    }
}
