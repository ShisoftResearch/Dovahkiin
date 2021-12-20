use bifrost::utils::serde::serialize;
use bifrost_hasher::{hash_bytes, hash_bytes_secondary};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde;
use std::io::{Cursor, Error};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct Id {
    pub higher: u64,
    pub lower: u64,
}

impl Id {
    pub const fn new(higher: u64, lower: u64) -> Id {
        Id { higher, lower }
    }
    pub fn from_obj<T>(obj: &T) -> Id
    where
        T: serde::Serialize,
    {
        let vec = serialize(obj);
        let bin = vec.as_slice();
        Id {
            higher: hash_bytes(&bin),
            lower: hash_bytes_secondary(&bin),
        }
    }
    pub fn is_greater_than(&self, other: &Id) -> bool {
        self.higher >= other.higher && self.lower > other.lower
    }
    pub const fn unit_id() -> Id {
        Id {
            higher: 0,
            lower: 0,
        }
    }
    pub const fn max_id() -> Id {
        Id { higher: !0, lower: !0 }
    }
    pub fn is_unit_id(&self) -> bool {
        self.higher == 0 && self.lower == 0
    }
    pub fn to_binary(&self) -> [u8; 16] {
        let mut slice = [0u8; 16];
        {
            let mut cursor = Cursor::new(&mut slice[..]);
            cursor.write_u64::<BigEndian>(self.higher).unwrap();
            cursor.write_u64::<BigEndian>(self.lower).unwrap();
        }
        return slice;
    }
    pub fn from_binary<T>(cursor: &mut Cursor<T>) -> Result<Id, Error>
    where
        Cursor<T>: ReadBytesExt,
    {
        Ok(Id::new(
            cursor.read_u64::<BigEndian>()?,
            cursor.read_u64::<BigEndian>()?,
        ))
    }
}

impl Default for Id {
    fn default() -> Id {
        Self::unit_id()
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use types::custom_types::id::Id;

    #[test]
    fn compare() {
        let id_1 = Id::new(1, 2);
        let id_2 = Id::new(2, 2);
        let id_3 = Id::new(1, 2);
        let id_4 = Id::new(1, 3);
        assert_eq!(id_1.cmp(&id_2), Ordering::Less);
        assert!(id_1 < id_2);
        assert_eq!(id_1, id_3);
        assert!(id_4 > id_1);
    }

    #[test]
    fn with_btree() {
        let mut map = BTreeMap::new();
        let id_1 = Id::new(1, 2);
        let id_2 = Id::new(2, 2);
        let id_3 = Id::new(1, 3);
        let id_4 = Id::new(1, 5);
        let id_5 = Id::new(3, 5);

        assert!(map.insert(id_1, 1).is_none());
        assert!(map.insert(id_2, 2).is_none());
        assert!(map.insert(id_3, 3).is_none());
        assert!(map.insert(id_4, 4).is_none());
        assert!(map.insert(id_5, 5).is_none());

        assert_eq!(map.get(&id_1), Some(&1));
        assert_eq!(map.get(&id_2), Some(&2));
        assert_eq!(map.get(&id_3), Some(&3));
        assert_eq!(map.get(&id_4), Some(&4));
        assert_eq!(map.get(&id_5), Some(&5));
    }

    #[test]
    fn with_hashmap() {
        let mut map = HashMap::new();
        let id_1 = Id::new(1, 2);
        let id_2 = Id::new(2, 2);
        let id_3 = Id::new(1, 3);
        let id_4 = Id::new(1, 5);
        let id_5 = Id::new(3, 5);

        assert!(map.insert(id_1, 1).is_none());
        assert!(map.insert(id_2, 2).is_none());
        assert!(map.insert(id_3, 3).is_none());
        assert!(map.insert(id_4, 4).is_none());
        assert!(map.insert(id_5, 5).is_none());

        assert_eq!(map.get(&id_1), Some(&1));
        assert_eq!(map.get(&id_2), Some(&2));
        assert_eq!(map.get(&id_3), Some(&3));
        assert_eq!(map.get(&id_4), Some(&4));
        assert_eq!(map.get(&id_5), Some(&5));
    }
}
