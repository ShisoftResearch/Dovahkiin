use bifrost::utils::bincode::{deserialize, serialize};
use serde;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Any {
    pub data: Vec<u8>,
}
impl Any {
    pub fn to<'a, T>(&'a self) -> T
    where
        T: serde::Deserialize<'a>,
    {
        deserialize(&self.data)
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
