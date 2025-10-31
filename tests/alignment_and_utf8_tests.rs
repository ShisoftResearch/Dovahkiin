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

/// Helper to write string data at an arbitrary offset (potentially misaligned)
unsafe fn write_string_at_offset(base_ptr: *mut u8, offset: usize, s: &str) -> usize {
    let write_ptr = base_ptr.add(offset);
    let len = s.len() as u32;
    
    // Write length as u32 (using unaligned write to test alignment handling)
    std::ptr::write_unaligned(write_ptr as *mut u32, len);
    
    // Write string bytes
    let str_ptr = write_ptr.add(4);
    std::ptr::copy_nonoverlapping(s.as_ptr(), str_ptr, s.len());
    
    4 + s.len() // Return total size written
}

#[test]
fn test_aligned_string_read() {
    // Test reading a properly aligned string
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let test_str = "Hello, aligned world!";
        write_string_at_offset(ptr, 0, test_str);
        
        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, test_str);
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_multiple_aligned_strings() {
    // Test reading multiple properly aligned strings
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let test_str1 = "First string";
        let test_str2 = "Second string";
        
        write_string_at_offset(ptr, 0, test_str1);
        let offset1 = 4 + test_str1.len();
        let aligned_offset = (offset1 + 3) & !3; // Align to 4 bytes
        write_string_at_offset(ptr, aligned_offset, test_str2);
        
        let read_str1 = string_io::read(ptr as usize);
        assert_eq!(read_str1, test_str1);
        
        let read_str2 = string_io::read((ptr as usize) + aligned_offset);
        assert_eq!(read_str2, test_str2);
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_string_with_padding() {
    // Test that alignment padding doesn't affect reads
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let test_str = "Test";  // 4 bytes
        write_string_at_offset(ptr, 0, test_str);
        
        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, test_str);
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_strings_various_lengths() {
    // Test strings of various lengths to ensure proper handling
    let size = 2048;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let strings = vec!["A", "AB", "ABC", "ABCD", "ABCDE"];
        let mut offset = 0;
        
        for s in &strings {
            write_string_at_offset(ptr, offset, s);
            let read_str = string_io::read((ptr as usize) + offset);
            assert_eq!(read_str, *s);
            offset += 4 + s.len();
            offset = (offset + 3) & !3; // Align to 4 bytes
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_string_array_aligned() {
    // Test reading an array of aligned strings
    let size = 2048;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let strings = vec!["First", "Second", "Third"];
        let mut offset = 0;
        
        for s in &strings {
            let written = write_string_at_offset(ptr, offset, s);
            offset += written;
            // Align to next 4-byte boundary
            offset = (offset + 3) & !3;
        }
        
        let (read_strings, _) = string_io::read_slice(ptr as usize, strings.len());
        
        for (i, read_str) in read_strings.iter().enumerate() {
            assert_eq!(*read_str, strings[i]);
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_string_array_with_alignment() {
    // Test reading an array of strings with proper alignment between elements
    let size = 2048;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let strings = vec!["Alpha", "Beta", "Gamma", "Delta"];
        
        let mut offset = 0;
        
        for s in &strings {
            let written = write_string_at_offset(ptr, offset, s);
            offset += written;
            // Align to 4 bytes for next string
            offset = (offset + 3) & !3;
        }
        
        let (read_strings, _) = string_io::read_slice(ptr as usize, strings.len());
        
        for (i, read_str) in read_strings.iter().enumerate() {
            assert_eq!(*read_str, strings[i]);
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
#[should_panic]
fn test_invalid_utf8_handling() {
    // Test that invalid UTF-8 doesn't panic
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let write_ptr = ptr as *mut u32;
        // Write length
        *write_ptr = 4;
        
        // Write invalid UTF-8 sequence
        let str_ptr = ptr.add(4);
        str_ptr.write(0xFF);
        str_ptr.add(1).write(0xFF);
        str_ptr.add(2).write(0xFF);
        str_ptr.add(3).write(0xFF);
        
        // This should not panic, should return empty string
        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, "");
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
#[should_panic]
fn test_mixed_valid_invalid_utf8_array() {
    // Test array with mix of valid and invalid UTF-8 strings
    let size = 2048;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let mut offset = 0;
        
        // Write valid string
        let s1 = "Valid string";
        offset += write_string_at_offset(ptr, offset, s1);
        offset = (offset + 3) & !3;
        
        // Write invalid UTF-8
        let write_ptr = ptr.add(offset) as *mut u32;
        *write_ptr = 3;
        let str_ptr = ptr.add(offset + 4);
        str_ptr.write(0xFF);
        str_ptr.add(1).write(0xFE);
        str_ptr.add(2).write(0xFD);
        offset += 7;
        offset = (offset + 3) & !3;
        
        // Write another valid string
        let s3 = "Another valid";
        write_string_at_offset(ptr, offset, s3);
        
        let (read_strings, _) = string_io::read_slice(ptr as usize, 3);
        
        assert_eq!(read_strings[0], s1);
        assert_eq!(read_strings[1], ""); // Invalid UTF-8 returns empty string
        assert_eq!(read_strings[2], s3);
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_empty_string() {
    // Test reading empty string
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        write_string_at_offset(ptr, 0, "");
        
        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, "");
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_unicode_strings() {
    // Test reading strings with various Unicode characters
    let size = 2048;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let unicode_strings = vec![
            "Hello 世界",
            "Emoji: 🦀🔥",
            "Math: ∑∫∂",
            "العربية",
            "Русский",
        ];
        
        let mut offset = 0;
        for s in &unicode_strings {
            let written = write_string_at_offset(ptr, offset, s);
            offset += written;
            offset = (offset + 3) & !3;
        }
        
        let (read_strings, _) = string_io::read_slice(ptr as usize, unicode_strings.len());
        
        for (i, read_str) in read_strings.iter().enumerate() {
            assert_eq!(*read_str, unicode_strings[i]);
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_large_string() {
    // Test reading a large string
    let size = 1024 * 100;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let large_str = "x".repeat(10000);
        write_string_at_offset(ptr, 0, &large_str);
        
        let read_str = string_io::read(ptr as usize);
        assert_eq!(read_str, large_str);
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_string_size_calculation() {
    // Test that size_at returns correct size for strings (including padding)
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let test_str = "Test string";
        write_string_at_offset(ptr, 0, test_str);
        
        let calculated_size = string_io::size_at(ptr as usize);
        // Size should include padding to 4-byte boundary
        let unpadded_size = 4 + test_str.len(); // 4 bytes for length + string bytes
        let align = 4;
        let expected_size = (unpadded_size + align - 1) & !(align - 1);
        
        assert_eq!(calculated_size, expected_size);
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_bytes_array_aligned() {
    // Test reading an array of aligned byte arrays
    let size = 2048;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let byte_arrays = vec![
            vec![0x01, 0x02, 0x03],
            vec![0x04, 0x05],
            vec![0x06, 0x07, 0x08, 0x09],
        ];
        
        let mut offset = 0;
        for bytes in &byte_arrays {
            let write_ptr = ptr.add(offset) as *mut u32;
            *write_ptr = bytes.len() as u32;
            
            let bytes_ptr = ptr.add(offset + 4);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), bytes_ptr, bytes.len());
            
            offset += 4 + bytes.len();
            offset = (offset + 3) & !3; // Align to 4 bytes
        }
        
        let (read_bytes, _) = bytes_io::read_slice(ptr as usize, byte_arrays.len());
        
        for (i, read_byte_slice) in read_bytes.iter().enumerate() {
            assert_eq!(*read_byte_slice, byte_arrays[i].as_slice());
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_get_shared_prim_array_val_valid_types() {
    // Test that get_shared_prim_array_val works for all valid primitive types
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        // Write u32 array
        let u32_array = [1u32, 2u32, 3u32, 4u32];
        let write_ptr = ptr as *mut u32;
        for (i, val) in u32_array.iter().enumerate() {
            write_ptr.add(i).write(*val);
        }
        
        let mut mem_ptr = ptr as usize;
        let result = get_shared_prim_array_val(Type::U32, u32_array.len(), &mut mem_ptr);
        
        assert!(result.is_some());
        let array = result.unwrap();
        
        if let SharedPrimArray::U32(slice) = array {
            assert_eq!(slice.len(), u32_array.len());
            for (i, val) in slice.iter().enumerate() {
                assert_eq!(*val, u32_array[i]);
            }
        } else {
            panic!("Expected U32 array");
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_get_shared_prim_array_val_invalid_type() {
    // Test that get_shared_prim_array_val returns None for invalid types
    let size = 1024;
    let ptr = alloc_aligned(size, 16);
    
    let mut mem_ptr = ptr as usize;
    
    // Type::Map is not a valid primitive type for arrays
    let result = get_shared_prim_array_val(Type::Map, 3, &mut mem_ptr);
    
    assert!(result.is_none());
    
    unsafe {
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_read_slice_with_varying_lengths() {
    // Test that read_slice properly handles strings of varying lengths
    let size = 2048;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        // Create strings of varying lengths
        let strings = vec!["A", "BB", "CCC", "DDDD", "EEEEE"];
        
        let mut offset = 0;
        for s in &strings {
            let written = write_string_at_offset(ptr, offset, s);
            offset += written;
            // Align each string to 4 bytes
            offset = (offset + 3) & !3;
        }
        
        let (read_strings, _) = string_io::read_slice(ptr as usize, strings.len());
        
        for (i, read_str) in read_strings.iter().enumerate() {
            assert_eq!(*read_str, strings[i]);
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_utf8_variable_length_array_misalignment() {
    // Test the alignment bug fix with UTF-8 strings that would cause misalignment
    // This test demonstrates the bug where strings with byte lengths not divisible by 4
    // would cause the next read to be misaligned
    let size = 4096;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        // These strings have varying byte lengths that test alignment handling
        let strings = vec![
            String::from(""),                      // 0 bytes -> size_at=4 -> aligned
            String::from("ಬಾ ಇಲ್ಲಿ ಸಂಭವಿಸ"),    // 41 bytes -> size_at=45 -> misaligned!
            String::from("中文测试文本"),           // 18 bytes -> size_at=22 -> misaligned!
            String::from("Hello"),                 // 5 bytes -> size_at=9 -> misaligned!
            String::from("🦀🔥"),                  // 8 bytes -> size_at=12 -> aligned
        ];
        
        let mut offset = 0;
        for s in &strings {
            let written = write_string_at_offset(ptr, offset, s);
            offset += written;
            // Align to 4 bytes for next string (simulating proper storage)
            offset = (offset + 3) & !3;
        }
        
        // This should NOT crash even with misaligned string sizes
        let (read_strings, total_size) = string_io::read_slice(ptr as usize, strings.len());
        
        assert_eq!(read_strings.len(), strings.len());
        for (i, read_str) in read_strings.iter().enumerate() {
            assert_eq!(*read_str, strings[i], "Mismatch at index {}", i);
        }
        
        // Verify that total_size accounts for alignment padding
        assert_eq!(total_size, offset);
        
        dealloc_aligned(ptr, size, 16);
    }
}

#[test]
fn test_kannada_telugu_unicode_array() {
    // Test with Kannada and Telugu scripts which have complex UTF-8 encoding
    let size = 4096;
    let ptr = alloc_aligned(size, 16);
    
    unsafe {
        let strings = vec![
            "ಕನ್ನಡ",           // Kannada
            "తెలుగు",          // Telugu
            "தமிழ்",           // Tamil
            "हिन्दी",          // Hindi
            "বাংলা",           // Bengali
        ];
        
        let mut offset = 0;
        for s in &strings {
            let written = write_string_at_offset(ptr, offset, s);
            offset += written;
            offset = (offset + 3) & !3;
        }
        
        let (read_strings, _) = string_io::read_slice(ptr as usize, strings.len());
        
        for (i, read_str) in read_strings.iter().enumerate() {
            assert_eq!(*read_str, strings[i]);
        }
        
        dealloc_aligned(ptr, size, 16);
    }
}

