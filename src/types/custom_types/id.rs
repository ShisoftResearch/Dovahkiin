use std::cmp::Ordering;
use bifrost::utils::bincode::{serialize, deserialize};
use bifrost_hasher::{hash_bytes, hash_bytes_secondary};
use serde;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash)]
pub struct Id {
    pub higher: u64,
    pub lower:  u64,
}

impl Id {
    pub fn new(higher: u64, lower: u64) -> Id {
        Id {
            higher: higher,
            lower: lower,
        }
    }
    // pub fn rand() -> Id { // TODO: use trait in neb
    //     let (hi, lw) = rand::next_two();
    //     Id::new(hi, lw)
    // }
    // pub fn from_header(header: &Header) -> Id { // TODO: use trait in neb
    //     Id {
    //         higher: header.partition,
    //         lower: header.hash
    //     }
    // }
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