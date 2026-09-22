//! Module for cryptographic hashing and message authentication.
//!
//! Implemented natively by the runner (Rust `md-5` / `sha1` / `sha2` / `hmac`
//! crates) — no wasm-host round trip. Every digest is returned as a lowercase
//! hex string, the format most streaming sites expect when signing request
//! parameters. The API mirrors aidoku's `imports::crypto`.
use super::{FFIResult, Rid};
use crate::alloc::String;
use crate::imports::std::read_string_and_destroy;

#[link(wasm_import_module = "crypto")]
unsafe extern "C" {
	#[link_name = "md5"]
	fn _md5(ptr: *const u8, len: usize) -> FFIResult;

	#[link_name = "sha1"]
	fn _sha1(ptr: *const u8, len: usize) -> FFIResult;

	#[link_name = "sha256"]
	fn _sha256(ptr: *const u8, len: usize) -> FFIResult;

	#[link_name = "hmac_sha1"]
	fn _hmac_sha1(data_ptr: *const u8, data_len: usize, key_ptr: *const u8, key_len: usize)
		-> FFIResult;

	#[link_name = "hmac_sha256"]
	fn _hmac_sha256(data_ptr: *const u8, data_len: usize, key_ptr: *const u8, key_len: usize)
		-> FFIResult;
}

/// MD5 digest of `data`, lowercase hex.
pub fn md5(data: &str) -> String {
	hex_result(unsafe { _md5(data.as_ptr(), data.len()) })
}

/// SHA-1 digest of `data`, lowercase hex.
pub fn sha1(data: &str) -> String {
	hex_result(unsafe { _sha1(data.as_ptr(), data.len()) })
}

/// SHA-256 digest of `data`, lowercase hex.
pub fn sha256(data: &str) -> String {
	hex_result(unsafe { _sha256(data.as_ptr(), data.len()) })
}

/// HMAC-SHA1 of `data` keyed by `key`, lowercase hex.
pub fn hmac_sha1(data: &str, key: &str) -> String {
	hex_result(unsafe { _hmac_sha1(data.as_ptr(), data.len(), key.as_ptr(), key.len()) })
}

/// HMAC-SHA256 of `data` keyed by `key`, lowercase hex.
pub fn hmac_sha256(data: &str, key: &str) -> String {
	hex_result(unsafe { _hmac_sha256(data.as_ptr(), data.len(), key.as_ptr(), key.len()) })
}

fn hex_result(rid: Rid) -> String {
	read_string_and_destroy(rid).unwrap_or_default()
}