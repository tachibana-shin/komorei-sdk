//! Module for base64 encoding and decoding.
//!
//! Implemented natively by the runner (Rust `base64` crate) — sources never
//! have to carry their own decoder, which keeps the wasm smaller and the
//! decode faster. The runner tolerates all four common variants (standard
//! padded/unpadded and URL-safe padded/unpadded), matching what real grab
//! pages ship.
use super::FFIResult;
use crate::alloc::{String, Vec};
use crate::imports::std::{destroy, read_buffer, read_string_and_destroy};

#[link(wasm_import_module = "base64")]
unsafe extern "C" {
	#[link_name = "encode"]
	fn _encode(ptr: *const u8, len: usize) -> FFIResult;

	#[link_name = "decode"]
	fn _decode(ptr: *const u8, len: usize) -> FFIResult;
}

/// Encode raw bytes as standard (RFC 4648 §4) base64 with padding.
pub fn encode(data: &[u8]) -> String {
	let rid = unsafe { _encode(data.as_ptr(), data.len()) };
	// Base64 output is ASCII; the empty result maps to the empty string too.
	read_string_and_destroy(rid).unwrap_or_default()
}

/// Decode a base64 string (any tolerated variant) into raw bytes.
///
/// Returns `None` when the input is not valid base64.
pub fn decode(data: &str) -> Option<Vec<u8>> {
	let rid = unsafe { _decode(data.as_ptr(), data.len()) };
	if rid < 0 {
		return None;
	}
	let buffer = read_buffer(rid);
	unsafe { destroy(rid) };
	buffer
}