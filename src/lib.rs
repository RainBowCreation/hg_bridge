// In: SharedLogic/src/lib.rs

use sha3::{Digest, Sha3_256};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// --- Main Application Logic Function ---

/// This is our core, shared hashing function for client/server synchronization.
/// It deterministically combines a timestamp (in milliseconds) and a secret string
/// to produce a SHA3-256 hash.
pub fn hash_random(time_ms: u64, secret: &str) -> String {
    let mut data_to_hash = Vec::new();
    data_to_hash.extend_from_slice(&time_ms.to_le_bytes());
    data_to_hash.extend_from_slice(secret.as_bytes());

    hash_bytes(&data_to_hash)
}

// --- General Purpose Utility Function ---

/// A general-purpose hashing utility function.
/// It takes any slice of bytes and returns a SHA3-256 hash string.
/// This is used internally by the server to generate a new secret from a seed.
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}


// --- FFI (Foreign Function Interface) Section ---
// These functions allow the compiled WASM module to be called from other languages.

/// FFI wrapper for our main time-sensitive hashing function.
#[no_mangle]
pub extern "C" fn hash_random_ffi(time_ms: u64, secret_ptr: *const c_char) -> *mut c_char {
    let secret = if secret_ptr.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(secret_ptr) }.to_str().unwrap_or("")
    };
    let result = hash_random(time_ms, secret);
    let c_string = CString::new(result).unwrap();
    c_string.into_raw()
}

/// FFI wrapper for our general-purpose hashing utility.
/// Takes a pointer to a C-style string and hashes its raw byte content.
/// Returns a pointer to a new C-style string containing the hash result.
#[no_mangle]
pub extern "C" fn hash_bytes_ffi(data_ptr: *const c_char) -> *mut c_char {
    // Safely handle the case where a null pointer is passed.
    if data_ptr.is_null() {
        let result = hash_bytes(&[]);
        return CString::new(result).unwrap().into_raw();
    }
    
    // Safely convert the C string pointer to a Rust byte slice.
    let c_str = unsafe { CStr::from_ptr(data_ptr) };
    let bytes = c_str.to_bytes();

    // Call our core utility function.
    let result = hash_bytes(bytes);

    // Convert the Rust String result back into a C string pointer for the caller.
    let c_string = CString::new(result).unwrap();
    c_string.into_raw()
}

/// Frees the memory for a C-style string that was created by our Rust code.
#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

pub fn to_u64(x: i64) -> u64 {
    ((x as u64) ^ (1 << 63)) & (1 << 63) | (x as u64 & (u64::MAX >> 1))
}
pub fn to_i64(x: u64) -> i64 {
    ((x as i64) ^ (1 << 63)) & (1 << 63) | (x & (u64::MAX >> 1)) as i64
}