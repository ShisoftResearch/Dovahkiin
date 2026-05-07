use dovahkiin::types::*;
use std::alloc::{alloc, dealloc, Layout};

/// Helper to allocate aligned memory for testing
fn alloc_aligned(size: usize, align: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).unwrap();
    unsafe { alloc(layout) }
}

/// Helper to deallocate aligned memory
fn dealloc_aligned(ptr: *mut u8, size: usize, align: usize) {
    let layout = Layout::from_size_align(size, align).unwrap();
    unsafe { dealloc(ptr, layout) }
}

// ============================================================================
// Primitive Type IO Tests
// ============================================================================

#[test]
fn test_bool_io() {
    let size = 64;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        bool_io::write(&true, ptr as usize);
        let read_val = *bool_io::read(ptr as usize);
        assert_eq!(read_val, true);

        bool_io::write(&false, (ptr as usize) + 8);
        let read_val2 = *bool_io::read((ptr as usize) + 8);
        assert_eq!(read_val2, false);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_i8_io() {
    let size = 64;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [i8::MIN, -42i8, 0i8, 42i8, i8::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * i8_io::type_size();
            i8_io::write(val, ptr as usize + offset);
            let read_val = *i8_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_i16_io() {
    let size = 128;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [i16::MIN, -1000i16, 0i16, 1000i16, i16::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * i16_io::type_size();
            i16_io::write(val, ptr as usize + offset);
            let read_val = *i16_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_i32_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [i32::MIN, -100000i32, 0i32, 100000i32, i32::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * i32_io::type_size();
            i32_io::write(val, ptr as usize + offset);
            let read_val = *i32_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_i64_io() {
    let size = 512;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [i64::MIN, -1000000000i64, 0i64, 1000000000i64, i64::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * i64_io::type_size();
            i64_io::write(val, ptr as usize + offset);
            let read_val = *i64_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_u8_io() {
    let size = 64;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [0u8, 42u8, 128u8, 200u8, u8::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * u8_io::type_size();
            u8_io::write(val, ptr as usize + offset);
            let read_val = *u8_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_u16_io() {
    let size = 128;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [0u16, 1000u16, 32768u16, 50000u16, u16::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * u16_io::type_size();
            u16_io::write(val, ptr as usize + offset);
            let read_val = *u16_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_u32_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [0u32, 100000u32, 2147483648u32, 3000000000u32, u32::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * u32_io::type_size();
            u32_io::write(val, ptr as usize + offset);
            let read_val = *u32_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_u64_io() {
    let size = 512;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [0u64, 1000000000u64, u64::MAX / 2, u64::MAX - 1, u64::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * u64_io::type_size();
            u64_io::write(val, ptr as usize + offset);
            let read_val = *u64_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_f32_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [0.0f32, 3.14f32, -2.718f32, f32::MIN, f32::MAX];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * f32_io::type_size();
            f32_io::write(val, ptr as usize + offset);
            let read_val = *f32_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_f64_io() {
    let size = 512;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [
            0.0f64,
            3.141592653589793f64,
            -2.718281828f64,
            f64::MIN,
            f64::MAX,
        ];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * f64_io::type_size();
            f64_io::write(val, ptr as usize + offset);
            let read_val = *f64_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_char_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = ['A', 'z', '0', '!', '🦀'];
        for (i, val) in test_vals.iter().enumerate() {
            let offset = i * char_io::type_size();
            char_io::write(val, ptr as usize + offset);
            let read_val = *char_io::read(ptr as usize + offset);
            assert_eq!(read_val, *val);
        }

        dealloc_aligned(ptr, size, 8);
    }
}

// ============================================================================
// Compound Type IO Tests
// ============================================================================

#[test]
fn test_pos2d32_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let pos = Pos2d32 { x: 1.5, y: 2.5 };
        pos2d32_io::write(&pos, ptr as usize);
        let read_pos = *pos2d32_io::read(ptr as usize);
        assert_eq!(read_pos.x, pos.x);
        assert_eq!(read_pos.y, pos.y);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_pos2d64_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let pos = Pos2d64 { x: 1.5, y: 2.5 };
        pos2d64_io::write(&pos, ptr as usize);
        let read_pos = *pos2d64_io::read(ptr as usize);
        assert_eq!(read_pos.x, pos.x);
        assert_eq!(read_pos.y, pos.y);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_pos3d32_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let pos = Pos3d32 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        pos3d32_io::write(&pos, ptr as usize);
        let read_pos = *pos3d32_io::read(ptr as usize);
        assert_eq!(read_pos.x, pos.x);
        assert_eq!(read_pos.y, pos.y);
        assert_eq!(read_pos.z, pos.z);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_pos3d64_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let pos = Pos3d64 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        pos3d64_io::write(&pos, ptr as usize);
        let read_pos = *pos3d64_io::read(ptr as usize);
        assert_eq!(read_pos.x, pos.x);
        assert_eq!(read_pos.y, pos.y);
        assert_eq!(read_pos.z, pos.z);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_id_io() {
    let size = 256;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let id = Id {
            higher: 123456789,
            lower: 987654321,
        };
        id_io::write(&id, ptr as usize);
        let read_id = *id_io::read(ptr as usize);
        assert_eq!(read_id.higher, id.higher);
        assert_eq!(read_id.lower, id.lower);

        dealloc_aligned(ptr, size, 8);
    }
}

// ============================================================================
// Variable Type IO Tests
// ============================================================================

#[test]
fn test_string_io_simple() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_str = "Hello, World!";

        // Write length
        u32_io::write(&(test_str.len() as u32), ptr as usize);

        // Write string bytes
        let str_ptr = ptr.add(4);
        std::ptr::copy_nonoverlapping(test_str.as_ptr(), str_ptr, test_str.len());

        // Read back
        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, test_str);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_string_io_empty() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        u32_io::write(&0u32, ptr as usize);
        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, "");

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_string_io_unicode() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_str = "Hello 世界 🦀";

        u32_io::write(&(test_str.len() as u32), ptr as usize);
        let str_ptr = ptr.add(4);
        std::ptr::copy_nonoverlapping(test_str.as_ptr(), str_ptr, test_str.len());

        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, test_str);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_string_io_size_calculation() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_str = "Test size";
        u32_io::write(&(test_str.len() as u32), ptr as usize);
        let str_ptr = ptr.add(4);
        std::ptr::copy_nonoverlapping(test_str.as_ptr(), str_ptr, test_str.len());

        let calculated_size = string_io::size_at(ptr as usize);
        // Size should include padding to 4-byte boundary
        let unpadded_size = 4 + test_str.len();
        let align = 4;
        let expected_size = (unpadded_size + align - 1) & !(align - 1);
        assert_eq!(calculated_size, expected_size);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_bytes_io() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_bytes = vec![1u8, 2, 3, 4, 5, 0xFF, 0xAB, 0xCD];

        u32_io::write(&(test_bytes.len() as u32), ptr as usize);
        let bytes_ptr = ptr.add(4);
        std::ptr::copy_nonoverlapping(test_bytes.as_ptr(), bytes_ptr, test_bytes.len());

        let read_bytes = bytes_io::read(ptr as usize);
        assert_eq!(read_bytes, test_bytes.as_slice());

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_small_bytes_io() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_bytes = vec![1u8, 2, 3, 4, 5];

        u8_io::write(&(test_bytes.len() as u8), ptr as usize);
        let bytes_ptr = ptr.add(1);
        std::ptr::copy_nonoverlapping(test_bytes.as_ptr(), bytes_ptr, test_bytes.len());

        let read_bytes = small_bytes_io::read(ptr as usize);
        assert_eq!(read_bytes, test_bytes.as_slice());

        dealloc_aligned(ptr, size, 8);
    }
}

// ============================================================================
// Type Size Tests
// ============================================================================

#[test]
fn test_primitive_type_sizes() {
    assert_eq!(bool_io::type_size(), std::mem::size_of::<bool>());
    assert_eq!(i8_io::type_size(), 1);
    assert_eq!(i16_io::type_size(), 2);
    assert_eq!(i32_io::type_size(), 4);
    assert_eq!(i64_io::type_size(), 8);
    assert_eq!(u8_io::type_size(), 1);
    assert_eq!(u16_io::type_size(), 2);
    assert_eq!(u32_io::type_size(), 4);
    assert_eq!(u64_io::type_size(), 8);
    assert_eq!(f32_io::type_size(), 4);
    assert_eq!(f64_io::type_size(), 8);
    assert_eq!(char_io::type_size(), std::mem::size_of::<char>());
}

#[test]
fn test_compound_type_sizes() {
    assert_eq!(pos2d32_io::type_size(), std::mem::size_of::<Pos2d32>());
    assert_eq!(pos2d64_io::type_size(), std::mem::size_of::<Pos2d64>());
    assert_eq!(pos3d32_io::type_size(), std::mem::size_of::<Pos3d32>());
    assert_eq!(pos3d64_io::type_size(), std::mem::size_of::<Pos3d64>());
    assert_eq!(id_io::type_size(), std::mem::size_of::<Id>());
}

#[test]
fn test_fixed_size_flags() {
    assert!(bool_io::fixed_size());
    assert!(i32_io::fixed_size());
    assert!(u64_io::fixed_size());
    assert!(f32_io::fixed_size());
    assert!(pos2d32_io::fixed_size());
    assert!(id_io::fixed_size());

    assert!(!string_io::fixed_size());
    assert!(!bytes_io::fixed_size());
    assert!(!small_bytes_io::fixed_size());
}

#[test]
fn test_type_alignments() {
    assert_eq!(i8_io::type_align(), std::mem::align_of::<i8>());
    assert_eq!(i16_io::type_align(), std::mem::align_of::<i16>());
    assert_eq!(i32_io::type_align(), std::mem::align_of::<i32>());
    assert_eq!(i64_io::type_align(), std::mem::align_of::<i64>());
    assert_eq!(u32_io::type_align(), std::mem::align_of::<u32>());
    assert_eq!(f64_io::type_align(), std::mem::align_of::<f64>());
}

// ============================================================================
// Array IO Tests
// ============================================================================

#[test]
fn test_primitive_array_read_slice() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [1u32, 2u32, 3u32, 4u32, 5u32];
        let write_ptr = ptr as *mut u32;
        for (i, val) in test_vals.iter().enumerate() {
            write_ptr.add(i).write(*val);
        }

        let (read_slice, read_size) = u32_io::read_slice(ptr as usize, test_vals.len());
        assert_eq!(read_slice.len(), test_vals.len());
        for (i, val) in read_slice.iter().enumerate() {
            assert_eq!(*val, test_vals[i]);
        }
        assert_eq!(read_size, test_vals.len() * 4);

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_get_owned_prim_array_val() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [10u32, 20u32, 30u32, 40u32];
        let write_ptr = ptr as *mut u32;
        for (i, val) in test_vals.iter().enumerate() {
            write_ptr.add(i).write(*val);
        }

        let mut mem_ptr = ptr as usize;
        let result = get_owned_prim_array_val(Type::U32, test_vals.len(), &mut mem_ptr);

        assert!(result.is_some());
        if let Some(OwnedPrimArray::U32(arr)) = result {
            assert_eq!(arr.len(), test_vals.len());
            for (i, val) in arr.iter().enumerate() {
                assert_eq!(*val, test_vals[i]);
            }
        } else {
            panic!("Expected U32 array");
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_get_shared_prim_array_val_i64() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_vals = [100i64, -200i64, 300i64, -400i64];
        let write_ptr = ptr as *mut i64;
        for (i, val) in test_vals.iter().enumerate() {
            write_ptr.add(i).write(*val);
        }

        let mut mem_ptr = ptr as usize;
        let result = get_shared_prim_array_val(Type::I64, test_vals.len(), &mut mem_ptr);

        assert!(result.is_some());
        if let Some(SharedPrimArray::I64(slice)) = result {
            assert_eq!(slice.len(), test_vals.len());
            for (i, val) in slice.iter().enumerate() {
                assert_eq!(*val, test_vals[i]);
            }
        } else {
            panic!("Expected I64 array");
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_get_size() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        // Fixed size types
        assert_eq!(get_size(Type::I32, ptr as usize), 4);
        assert_eq!(get_size(Type::U64, ptr as usize), 8);
        assert_eq!(get_size(Type::F32, ptr as usize), 4);

        // Variable size type (string)
        let test_str = "Variable";
        u32_io::write(&(test_str.len() as u32), ptr as usize);
        assert_eq!(get_size(Type::String, ptr as usize), 4 + test_str.len());

        dealloc_aligned(ptr, size, 8);
    }
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_zero_length_arrays() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let mut mem_ptr = ptr as usize;
        let result = get_owned_prim_array_val(Type::U32, 0, &mut mem_ptr);

        assert!(result.is_some());
        if let Some(OwnedPrimArray::U32(arr)) = result {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected empty U32 array");
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_single_element_array() {
    let size = 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let test_val = 42u32;
        let write_ptr = ptr as *mut u32;
        write_ptr.write(test_val);

        let mut mem_ptr = ptr as usize;
        let result = get_owned_prim_array_val(Type::U32, 1, &mut mem_ptr);

        assert!(result.is_some());
        if let Some(OwnedPrimArray::U32(arr)) = result {
            assert_eq!(arr.len(), 1);
            assert_eq!(arr[0], test_val);
        } else {
            panic!("Expected single element array");
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_large_primitive_array() {
    let size = 100 * 1024;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        let count = 10000;
        let write_ptr = ptr as *mut u32;
        for i in 0..count {
            write_ptr.add(i).write(i as u32);
        }

        let mut mem_ptr = ptr as usize;
        let result = get_owned_prim_array_val(Type::U32, count, &mut mem_ptr);

        assert!(result.is_some());
        if let Some(OwnedPrimArray::U32(arr)) = result {
            assert_eq!(arr.len(), count);
            for (i, val) in arr.iter().enumerate() {
                assert_eq!(*val, i as u32);
            }
        } else {
            panic!("Expected large array");
        }

        dealloc_aligned(ptr, size, 8);
    }
}

#[test]
fn test_boundary_values_all_types() {
    let size = 2048;
    let ptr = alloc_aligned(size, 8);

    unsafe {
        // Test boundary values for all signed types
        let i8_vals = [i8::MIN, 0i8, i8::MAX];
        let i16_vals = [i16::MIN, 0i16, i16::MAX];
        let i32_vals = [i32::MIN, 0i32, i32::MAX];
        let i64_vals = [i64::MIN, 0i64, i64::MAX];

        // Test boundary values for all unsigned types
        let u8_vals = [u8::MIN, u8::MAX / 2, u8::MAX];
        let u16_vals = [u16::MIN, u16::MAX / 2, u16::MAX];
        let u32_vals = [u32::MIN, u32::MAX / 2, u32::MAX];
        let u64_vals = [u64::MIN, u64::MAX / 2, u64::MAX];

        // Write and read back all values
        let mut offset = 0;

        for val in i8_vals.iter() {
            i8_io::write(val, ptr as usize + offset);
            assert_eq!(*i8_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        for val in i16_vals.iter() {
            i16_io::write(val, ptr as usize + offset);
            assert_eq!(*i16_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        for val in i32_vals.iter() {
            i32_io::write(val, ptr as usize + offset);
            assert_eq!(*i32_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        for val in i64_vals.iter() {
            i64_io::write(val, ptr as usize + offset);
            assert_eq!(*i64_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        for val in u8_vals.iter() {
            u8_io::write(val, ptr as usize + offset);
            assert_eq!(*u8_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        for val in u16_vals.iter() {
            u16_io::write(val, ptr as usize + offset);
            assert_eq!(*u16_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        for val in u32_vals.iter() {
            u32_io::write(val, ptr as usize + offset);
            assert_eq!(*u32_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        for val in u64_vals.iter() {
            u64_io::write(val, ptr as usize + offset);
            assert_eq!(*u64_io::read(ptr as usize + offset), *val);
            offset += 8;
        }

        dealloc_aligned(ptr, size, 8);
    }
}
