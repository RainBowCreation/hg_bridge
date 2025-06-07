use sha3::{Sha3_256, Digest};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Internal logic: SHA3-256 hashing
pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Internal logic: Add two numbers
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// C ABI function: add two numbers (usable in C# and TS via WASM)
#[no_mangle]
pub extern "C" fn add_numbers_ffi(a: i32, b: i32) -> i32 {
    add_numbers(a, b)
}

/// C ABI function: hash string using SHA3-256 and return a pointer to a C string
#[no_mangle]
pub extern "C" fn hash_sha256_ffi(input: *const c_char) -> *mut c_char {
    // Convert C string to Rust &str
    let c_str = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input)
    };

    let result = hash_sha256(c_str.to_str().unwrap_or(""));
    let c_string = CString::new(result).unwrap();
    c_string.into_raw() // caller must free
}

/// Frees memory allocated for the returned C string
#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        let _ = CString::from_raw(s); // deallocates memory
    }
}