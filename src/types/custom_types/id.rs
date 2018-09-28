use std::cmp::Ordering;
use bifrost::utils::bincode::{serialize};
use bifrost_hasher::{hash_bytes, hash_bytes_secondary};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use serde;
use std::io::{Cursor, Error};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash)]
pub struct Id {
    pub higher: u64,
    pub lower:  u64,
}

impl Id {
    pub fn new(higher: u64, lower: u64) -> Id {
        Id {
            higher,
            lower,
        }
    }
    pub fn from_obj<T>(obj: &T) -> Id where T: serde::Serialize {
        let vec = serialize(obj);
        let bin = vec.as_slice();
        Id {
            higher: hash_bytes(&bin),
            lower: hash_bytes_secondary(&bin)
        }
    }
    pub fn is_greater_than(&self, other: &Id) -> bool {
        self.higher >= other.higher && self.lower > other.lower
    }
    pub fn unit_id() -> Id {
        Id {higher: 0, lower: 0}
    }
    pub fn is_unit_id(&self) -> bool {
        self.higher == 0 && self.lower == 0
    }
    pub fn to_binary(&self) -> [u8; 16] {
        let mut slice = [0u8; 16];
        {
            let mut cursor = Cursor::new(&mut slice[..]);
            cursor.write_u64::<BigEndian>(self.higher);
            cursor.write_u64::<BigEndian>(self.lower);
        }
        return slice;
    }
    pub fn from_binary<T>(cursor: &mut Cursor<T>) -> Result<Id, Error>
        where Cursor<T>: ReadBytesExt
    {
        Ok(Id::new(
            cursor.read_u64::<BigEndian>()?,
            cursor.read_u64::<BigEndian>()?))
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.higher == other.higher && self.lower == other.lower
    }
    fn ne(&self, other: &Id) -> bool {
        self.higher != other.higher || self.lower != other.lower
    }
}
impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {return Some(Ordering::Equal)}
        if self.is_greater_than(other) {return Some(Ordering::Greater)}
        Some(Ordering::Less)
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {return Ordering::Equal}
        if self.is_greater_than(other) {return Ordering::Greater}
        Ordering::Less
    }
}