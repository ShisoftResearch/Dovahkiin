use dovahkiin::types::*;
use dovahkiin::{data_map, data_map_value};

#[test]
fn test_owned_value_basic_types() {
    // Test all basic numeric types
    assert_eq!(OwnedValue::Bool(true), OwnedValue::Bool(true));
    assert_eq!(OwnedValue::I8(42), OwnedValue::I8(42));
    assert_eq!(OwnedValue::I16(1000), OwnedValue::I16(1000));
    assert_eq!(OwnedValue::I32(100000), OwnedValue::I32(100000));
    assert_eq!(OwnedValue::I64(1000000000), OwnedValue::I64(1000000000));
    assert_eq!(OwnedValue::U8(255), OwnedValue::U8(255));
    assert_eq!(OwnedValue::U16(65535), OwnedValue::U16(65535));
    assert_eq!(OwnedValue::U32(4294967295), OwnedValue::U32(4294967295));
    assert_eq!(OwnedValue::U64(u64::MAX), OwnedValue::U64(u64::MAX));
    assert_eq!(OwnedValue::F32(3.14), OwnedValue::F32(3.14));
    assert_eq!(OwnedValue::F64(2.718281828), OwnedValue::F64(2.718281828));
    assert_eq!(OwnedValue::Char('A'), OwnedValue::Char('A'));
}

#[test]
fn test_owned_value_string() {
    let s = String::from("Hello, World!");
    let val = OwnedValue::String(s.clone());

    if let OwnedValue::String(extracted) = val {
        assert_eq!(extracted, s);
    } else {
        panic!("Expected String variant");
    }
}

#[test]
fn test_owned_value_array() {
    let arr = vec![OwnedValue::I32(1), OwnedValue::I32(2), OwnedValue::I32(3)];
    let val = OwnedValue::Array(arr.clone());

    if let OwnedValue::Array(extracted) = val {
        assert_eq!(extracted, arr);
    } else {
        panic!("Expected Array variant");
    }
}

#[test]
fn test_owned_value_prim_array() {
    let arr = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    let prim_arr = OwnedPrimArray::U32(arr.clone());
    let val = OwnedValue::PrimArray(prim_arr);

    if let OwnedValue::PrimArray(OwnedPrimArray::U32(extracted)) = val {
        assert_eq!(extracted, arr);
    } else {
        panic!("Expected U32 primitive array");
    }
}

#[test]
fn test_owned_value_map() {
    let mut map = OwnedMap::new();
    map.insert_value("key1", OwnedValue::I32(42));
    map.insert_value("key2", OwnedValue::String("value".to_string()));

    let val = OwnedValue::Map(map.clone());

    if let OwnedValue::Map(extracted) = val {
        assert_eq!(extracted.get("key1").i32(), Some(&42));
        assert_eq!(extracted.get("key2").string(), Some(&"value".to_string()));
    } else {
        panic!("Expected Map variant");
    }
}

#[test]
fn test_owned_value_null() {
    assert_eq!(OwnedValue::Null, OwnedValue::Null);
}

#[test]
fn test_owned_value_na() {
    assert_eq!(OwnedValue::NA, OwnedValue::NA);
}

#[test]
fn test_type_enum() {
    // Type enum starts with Null=0, Map=1, then defined types
    assert_eq!(Type::Null.id(), 0);
    assert_eq!(Type::Map.id(), 1);
    assert!(Type::Bool.id() > Type::Map.id());
    assert_ne!(Type::Bool, Type::I32);
    assert_ne!(Type::String, Type::I32);
}

#[test]
fn test_get_type_id() {
    assert_eq!(get_type_id("bool".to_string()), Type::Bool.id());
    assert_eq!(get_type_id("i32".to_string()), Type::I32.id());
    assert_eq!(get_type_id("string".to_string()), Type::String.id());
    assert_eq!(get_type_id("unknown".to_string()), 0);
}

#[test]
fn test_get_type() {
    assert_eq!(get_type(Type::Bool), "bool");
    assert_eq!(get_type(Type::I32), "i32");
    assert_eq!(get_type(Type::String), "string");
}

#[test]
fn test_owned_value_get_int() {
    assert_eq!(OwnedValue::I8(42).get_int(), Some(42));
    assert_eq!(OwnedValue::I16(-100).get_int(), Some(-100));
    assert_eq!(OwnedValue::I32(1000).get_int(), Some(1000));
    assert_eq!(OwnedValue::I64(-50000).get_int(), Some(-50000));
    assert_eq!(OwnedValue::U8(200).get_int(), Some(200));
    assert_eq!(OwnedValue::U16(50000).get_int(), Some(50000));
    assert_eq!(
        OwnedValue::String("not a number".to_string()).get_int(),
        None
    );
}

#[test]
fn test_owned_value_get_uint() {
    assert_eq!(OwnedValue::U8(42).get_uint(), Some(42));
    assert_eq!(OwnedValue::U16(1000).get_uint(), Some(1000));
    assert_eq!(OwnedValue::U32(100000).get_uint(), Some(100000));
    assert_eq!(OwnedValue::U64(1000000).get_uint(), Some(1000000));
    assert_eq!(OwnedValue::I32(-5).get_uint(), None);
    assert_eq!(
        OwnedValue::String("not a number".to_string()).get_uint(),
        None
    );
}

#[test]
fn test_pos2d32() {
    let pos = Pos2d32 { x: 1.5, y: 2.5 };
    assert_eq!(pos.x, 1.5);
    assert_eq!(pos.y, 2.5);
}

#[test]
fn test_pos3d32() {
    let pos = Pos3d32 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(pos.x, 1.0);
    assert_eq!(pos.y, 2.0);
    assert_eq!(pos.z, 3.0);
}

#[test]
fn test_id() {
    let id1 = Id {
        higher: 123,
        lower: 456,
    };
    let id2 = Id {
        higher: 123,
        lower: 456,
    };
    let id3 = Id {
        higher: 789,
        lower: 101,
    };

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_id_serialization() {
    let id = Id {
        higher: u64::MAX,
        lower: u64::MAX,
    };
    let bytes = id.to_binary();
    assert_eq!(bytes.len(), 16);

    let mut cursor = std::io::Cursor::new(&bytes[..]);
    let deserialized = Id::from_binary(&mut cursor).unwrap();
    assert_eq!(id, deserialized);
}

#[test]
fn test_bytes_type() {
    let data = vec![1u8, 2, 3, 4, 5];
    let bytes = Bytes { data: data.clone() };
    assert_eq!(bytes.data, data);
}

#[test]
fn test_small_bytes_type() {
    let data = vec![1u8, 2, 3];
    let small_bytes = SmallBytes { data: data.clone() };
    assert_eq!(small_bytes.data.as_slice(), data.as_slice());
}

#[test]
fn test_owned_map_operations() {
    let mut map = OwnedMap::new();

    // Test insertion
    map.insert_value("key1", OwnedValue::I32(42));
    map.insert_value("key2", OwnedValue::String("hello".to_string()));

    // Test retrieval
    assert_eq!(map.get("key1").i32(), Some(&42));
    assert_eq!(map.get("key2").string(), Some(&"hello".to_string()));

    // Test non-existent key
    assert_eq!(map.get("nonexistent"), &OwnedValue::Null);

    // Test existence by checking if value is not Null
    assert!(*map.get("key1") != OwnedValue::Null);
    assert!(*map.get("nonexistent") == OwnedValue::Null);
}

#[test]
fn test_owned_map_nested() {
    let mut inner_map = OwnedMap::new();
    inner_map.insert_value("inner_key", OwnedValue::I32(100));

    let mut outer_map = OwnedMap::new();
    outer_map.insert_value("outer_key", OwnedValue::Map(inner_map));

    if let OwnedValue::Map(inner) = outer_map.get("outer_key") {
        assert_eq!(inner.get("inner_key").i32(), Some(&100));
    } else {
        panic!("Expected nested map");
    }
}

#[test]
fn test_owned_map_debug_output() {
    let mut map = OwnedMap::new();
    map.insert_value("name", OwnedValue::String("Alice".to_string()));
    map.insert_value("age", OwnedValue::I32(30));
    map.insert_value("active", OwnedValue::Bool(true));

    let debug_output = format!("{:?}", map);

    // Verify the debug output contains the map contents
    assert!(debug_output.contains("name"));
    assert!(debug_output.contains("Alice"));
    assert!(debug_output.contains("age"));
    assert!(debug_output.contains("30"));
    assert!(debug_output.contains("active"));
    assert!(debug_output.contains("true"));
    assert!(debug_output.starts_with("{"));
    assert!(debug_output.ends_with("}"));

    // Print it so we can see it in test output
    println!("OwnedMap debug output: {}", debug_output);
}

#[test]
fn test_shared_map_debug_output() {
    let mut owned_map = OwnedMap::new();
    owned_map.insert_value("x", OwnedValue::I32(10));
    owned_map.insert_value("y", OwnedValue::I32(20));

    let shared_map = owned_map.shared();
    let debug_output = format!("{:?}", shared_map);

    // Verify the debug output contains the map contents
    assert!(debug_output.contains("x"));
    assert!(debug_output.contains("10"));
    assert!(debug_output.contains("y"));
    assert!(debug_output.contains("20"));
    assert!(debug_output.starts_with("{"));
    assert!(debug_output.ends_with("}"));

    // Print it so we can see it in test output
    println!("SharedMap debug output: {}", debug_output);
}

#[test]
fn test_prim_array_sizes() {
    let u8_arr = OwnedPrimArray::U8(vec![1u8, 2, 3, 4]);
    let u32_arr = OwnedPrimArray::U32(vec![1u32, 2, 3, 4]);
    let u64_arr = OwnedPrimArray::U64(vec![1u64, 2, 3, 4]);

    assert_eq!(u8_arr.len(), 4);
    assert_eq!(u32_arr.len(), 4);
    assert_eq!(u64_arr.len(), 4);
}

#[test]
fn test_prim_array_features() {
    let arr = OwnedPrimArray::U32(vec![42u32, 100u32, 255u32]);
    let features = arr.features();
    assert_eq!(features.len(), 3);
}

#[test]
fn test_prim_array_hashes() {
    let arr = OwnedPrimArray::U32(vec![1u32, 2u32, 3u32]);
    let hashes = arr.hashes();
    assert_eq!(hashes.len(), 3);
}

#[test]
fn test_data_map_macro() {
    let map = data_map! {
        x: OwnedValue::I32(10),
        y: OwnedValue::String("test".to_string())
    };

    assert_eq!(map.get("x").i32(), Some(&10));
    assert_eq!(map.get("y").string(), Some(&"test".to_string()));
}

#[test]
fn test_data_map_value_macro() {
    let value = data_map_value! {
        a: OwnedValue::Bool(true),
        b: OwnedValue::F64(3.14)
    };

    if let OwnedValue::Map(map) = value {
        assert_eq!(map.get("a").bool(), Some(&true));
        assert_eq!(map.get("b").f64(), Some(&3.14));
    } else {
        panic!("Expected map value");
    }
}

#[test]
fn test_owned_value_clone() {
    let original = OwnedValue::String("test".to_string());
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_owned_value_array_nested() {
    let inner = vec![OwnedValue::I32(1), OwnedValue::I32(2)];
    let outer = vec![
        OwnedValue::Array(inner.clone()),
        OwnedValue::Array(inner.clone()),
    ];
    let nested = OwnedValue::Array(outer);

    if let OwnedValue::Array(outer_arr) = nested {
        assert_eq!(outer_arr.len(), 2);
        if let OwnedValue::Array(inner_arr) = &outer_arr[0] {
            assert_eq!(inner_arr.len(), 2);
        } else {
            panic!("Expected inner array");
        }
    } else {
        panic!("Expected outer array");
    }
}

#[test]
fn test_type_id_of() {
    assert_eq!(type_id_of(Type::Bool), Type::Bool as u32);
    assert_eq!(type_id_of(Type::I32), Type::I32 as u32);
    assert_eq!(type_id_of(Type::String), Type::String as u32);
}

#[test]
fn test_all_primitive_types() {
    // Ensure all primitive types are distinct
    let types = vec![
        Type::Bool,
        Type::Char,
        Type::I8,
        Type::I16,
        Type::I32,
        Type::I64,
        Type::U8,
        Type::U16,
        Type::U32,
        Type::U64,
        Type::F32,
        Type::F64,
    ];

    for (i, t1) in types.iter().enumerate() {
        for (j, t2) in types.iter().enumerate() {
            if i != j {
                assert_ne!(t1.id(), t2.id(), "Types {:?} and {:?} have same ID", t1, t2);
            }
        }
    }
}

#[test]
fn test_owned_prim_array_all_types() {
    // Test that we can create primitive arrays for all types
    let _bool_arr = OwnedPrimArray::Bool(vec![true, false]);
    let _i8_arr = OwnedPrimArray::I8(vec![1i8, 2, 3]);
    let _i16_arr = OwnedPrimArray::I16(vec![1i16, 2, 3]);
    let _i32_arr = OwnedPrimArray::I32(vec![1i32, 2, 3]);
    let _i64_arr = OwnedPrimArray::I64(vec![1i64, 2, 3]);
    let _u8_arr = OwnedPrimArray::U8(vec![1u8, 2, 3]);
    let _u16_arr = OwnedPrimArray::U16(vec![1u16, 2, 3]);
    let _u32_arr = OwnedPrimArray::U32(vec![1u32, 2, 3]);
    let _u64_arr = OwnedPrimArray::U64(vec![1u64, 2, 3]);
    let _f32_arr = OwnedPrimArray::F32(vec![1.0f32, 2.0, 3.0]);
    let _f64_arr = OwnedPrimArray::F64(vec![1.0f64, 2.0, 3.0]);
    let _char_arr = OwnedPrimArray::Char(vec!['a', 'b', 'c']);
}

#[test]
fn test_empty_collections() {
    let empty_arr = OwnedValue::Array(vec![]);
    if let OwnedValue::Array(arr) = empty_arr {
        assert_eq!(arr.len(), 0);
    }

    let empty_prim = OwnedValue::PrimArray(OwnedPrimArray::U32(vec![]));
    if let OwnedValue::PrimArray(OwnedPrimArray::U32(arr)) = empty_prim {
        assert_eq!(arr.len(), 0);
    }

    let empty_map = OwnedValue::Map(OwnedMap::new());
    if let OwnedValue::Map(map) = empty_map {
        assert!(*map.get("any_key") == OwnedValue::Null);
        assert_eq!(map.len(), 0);
    }
}

#[test]
fn test_large_numbers() {
    assert_eq!(OwnedValue::I64(i64::MAX).get_int(), Some(i64::MAX as isize));
    assert_eq!(OwnedValue::I64(i64::MIN).get_int(), Some(i64::MIN as isize));
    assert_eq!(
        OwnedValue::U64(u64::MAX).get_uint(),
        Some(u64::MAX as usize)
    );
}
