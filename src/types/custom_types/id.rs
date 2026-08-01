use bifrost::utils::serde::serialize;
use bifrost_hasher::hash_bytes;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde;
use std::io::{Cursor, Error};

/// Compact 64-bit cell id (see Nebuchadnezzar
/// `docs/superpowers/specs/2026-08-02-compact-cell-id-design.md`).
///
/// Two classes, separated by the top bit:
///
/// ```text
/// allocated:  [ tag=0 : 1 ][ locality : 15 ][ origin : 12 ][ sequence : 36 ]
/// hashed:     [ tag=1 : 1 ][ hash                                     : 63 ]
/// ```
///
/// Uniqueness of allocated ids rests entirely on `(origin, sequence)`;
/// the locality field is a free-form placement affinity key. Hashed ids
/// derive from content and are bounded by a per-domain collision budget
/// with detection at the storage layer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct Id(pub u64);

pub const ID_TAG_SHIFT: u32 = 63;
pub const ID_LOCALITY_BITS: u32 = 15;
pub const ID_ORIGIN_BITS: u32 = 12;
pub const ID_SEQUENCE_BITS: u32 = 36;
pub const ID_LOCALITY_SHIFT: u32 = ID_ORIGIN_BITS + ID_SEQUENCE_BITS; // 48
pub const ID_ORIGIN_SHIFT: u32 = ID_SEQUENCE_BITS; // 36
pub const ID_LOCALITY_MASK: u64 = (1 << ID_LOCALITY_BITS) - 1;
pub const ID_ORIGIN_MASK: u64 = (1 << ID_ORIGIN_BITS) - 1;
pub const ID_SEQUENCE_MASK: u64 = (1 << ID_SEQUENCE_BITS) - 1;
pub const ID_HASH_MASK: u64 = (1 << 63) - 1;
const ID_TAG_HASHED: u64 = 1 << ID_TAG_SHIFT;

impl Id {
    pub const fn from_bits(bits: u64) -> Id {
        Id(bits)
    }

    pub const fn bits(&self) -> u64 {
        self.0
    }

    /// Composes an allocated-class id. `locality` is the placement
    /// affinity key; uniqueness must be guaranteed by the caller's
    /// `(origin, sequence)` allocation discipline.
    pub const fn allocated(locality: u16, origin: u16, sequence: u64) -> Id {
        Id(((locality as u64 & ID_LOCALITY_MASK) << ID_LOCALITY_SHIFT)
            | ((origin as u64 & ID_ORIGIN_MASK) << ID_ORIGIN_SHIFT)
            | (sequence & ID_SEQUENCE_MASK))
    }

    /// Composes an allocated-class id co-located with `anchor`: the
    /// anchor's locality bits are copied onto a fresh
    /// `(origin, sequence)`. Placement affinity is fixed at creation.
    pub const fn allocated_near(anchor: &Id, origin: u16, sequence: u64) -> Id {
        Self::allocated(anchor.locality(), origin, sequence)
    }

    /// Composes a hashed-class id from a precomputed hash.
    pub const fn hashed(hash: u64) -> Id {
        Id(ID_TAG_HASHED | (hash & ID_HASH_MASK))
    }

    /// Hashed-class id derived from serialized content. Collision
    /// domains are per key-schema; the storage layer detects residual
    /// collisions by key comparison on access.
    pub fn from_obj<T>(obj: &T) -> Id
    where
        T: serde::Serialize,
    {
        let vec = serialize(obj);
        Self::hashed(hash_bytes(vec.as_slice()))
    }

    pub const fn is_hashed(&self) -> bool {
        self.0 & ID_TAG_HASHED != 0
    }

    /// Placement affinity key. For hashed ids this returns the top hash
    /// bits, which are uniform — so routing may use this accessor
    /// uniformly for both classes.
    pub const fn locality(&self) -> u16 {
        ((self.0 >> ID_LOCALITY_SHIFT) & ID_LOCALITY_MASK) as u16
    }

    /// Allocator identity of an allocated-class id.
    pub const fn origin(&self) -> u16 {
        ((self.0 >> ID_ORIGIN_SHIFT) & ID_ORIGIN_MASK) as u16
    }

    /// Per-origin sequence of an allocated-class id.
    pub const fn sequence(&self) -> u64 {
        self.0 & ID_SEQUENCE_MASK
    }

    /// Composes an allocated-class id from a legacy `(partition, sub)`
    /// pair: the partition folds into the locality bits and `sub` fills
    /// the low 48 bits, so consecutive `sub` values yield consecutive
    /// `bits()`. Intended for tests and migration shims; production ids
    /// come from the allocator or the hashed class.
    pub const fn from_parts(partition: u64, sub: u64) -> Id {
        Id(((partition & ID_LOCALITY_MASK) << ID_LOCALITY_SHIFT)
            | (sub & ((1 << ID_LOCALITY_SHIFT) - 1)))
    }

    pub const fn unit_id() -> Id {
        Id(0)
    }

    pub const fn max_id() -> Id {
        Id(!0)
    }

    pub fn is_unit_id(&self) -> bool {
        self.0 == 0
    }

    /// Big-endian bytes: byte-wise lexicographic order equals numeric
    /// `Ord`, which composed index keys rely on.
    pub fn to_binary(&self) -> [u8; 8] {
        let mut slice = [0u8; 8];
        {
            let mut cursor = Cursor::new(&mut slice[..]);
            cursor.write_u64::<BigEndian>(self.0).unwrap();
        }
        return slice;
    }

    pub fn from_binary<T>(cursor: &mut Cursor<T>) -> Result<Id, Error>
    where
        Cursor<T>: ReadBytesExt,
    {
        Ok(Id(cursor.read_u64::<BigEndian>()?))
    }

    pub fn into_option(self) -> Option<Id> {
        if self.is_unit_id() {
            None
        } else {
            Some(self)
        }
    }
}

impl Default for Id {
    fn default() -> Id {
        Self::unit_id()
    }
}

#[cfg(test)]
mod test {
    use crate::types::custom_types::id::*;
    use std::cmp::Ordering;
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    #[test]
    fn field_round_trip() {
        let id = Id::allocated(0x7ABC, 0xFED, 0x8_1234_5678);
        assert!(!id.is_hashed());
        assert_eq!(id.locality(), 0x7ABC);
        assert_eq!(id.origin(), 0xFED);
        assert_eq!(id.sequence(), 0x8_1234_5678);
    }

    #[test]
    fn affinity_copies_locality_only() {
        let anchor = Id::allocated(0x1111, 1, 42);
        let near = Id::allocated_near(&anchor, 7, 99);
        assert_eq!(near.locality(), anchor.locality());
        assert_eq!(near.origin(), 7);
        assert_eq!(near.sequence(), 99);
        assert_ne!(near, anchor);
    }

    #[test]
    fn hashed_class_is_tagged_and_disjoint() {
        let hashed = Id::hashed(0x1234);
        assert!(hashed.is_hashed());
        // An allocated id can never equal any hashed id.
        let allocated = Id::allocated(0x7FFF, 0xFFF, ID_SEQUENCE_MASK);
        assert!(!allocated.is_hashed());
        assert_ne!(hashed.0 & ID_TAG_HASHED, allocated.0 & ID_TAG_HASHED);
    }

    #[test]
    fn binary_order_matches_ord() {
        let a = Id::allocated(1, 2, 3);
        let b = Id::allocated(1, 2, 4);
        let c = Id::hashed(5);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert!(a.to_binary() < b.to_binary());
        assert!(b.to_binary() < c.to_binary());
        let mut cur = std::io::Cursor::new(a.to_binary().to_vec());
        assert_eq!(Id::from_binary(&mut cur).unwrap(), a);
    }

    #[test]
    fn with_btree() {
        let mut map = BTreeMap::new();
        let ids: Vec<_> = (0u64..5).map(|i| Id::allocated(1, 1, i)).collect();
        for (n, id) in ids.iter().enumerate() {
            assert!(map.insert(*id, n).is_none());
        }
        for (n, id) in ids.iter().enumerate() {
            assert_eq!(map.get(id), Some(&n));
        }
    }

    #[test]
    fn with_hashmap() {
        let mut map = HashMap::new();
        let ids: Vec<_> = (0u64..5).map(|i| Id::allocated(1, 2, i)).collect();
        for (n, id) in ids.iter().enumerate() {
            assert!(map.insert(*id, n).is_none());
        }
        for (n, id) in ids.iter().enumerate() {
            assert_eq!(map.get(id), Some(&n));
        }
    }
}
