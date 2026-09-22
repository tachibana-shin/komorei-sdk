use crate::{
	FFIResult, Ptr, WasmEnv,
	libs::StoreItem,
};
use digest::Digest;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;
use wasmer::FunctionEnvMut;

/// MD5 digest as lowercase hex.
pub fn md5(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(data) = env.data().read_bytes(&env, ptr, len) else {
		return -1;
	};
	store_hex(&mut env, Md5::digest(&data).as_slice())
}

/// SHA-1 digest as lowercase hex.
pub fn sha1(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(data) = env.data().read_bytes(&env, ptr, len) else {
		return -1;
	};
	store_hex(&mut env, Sha1::digest(&data).as_slice())
}

/// SHA-256 digest as lowercase hex.
pub fn sha256(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(data) = env.data().read_bytes(&env, ptr, len) else {
		return -1;
	};
	store_hex(&mut env, Sha256::digest(&data).as_slice())
}

/// HMAC-SHA1 of `data` keyed by `key`, lowercase hex.
pub fn hmac_sha1(
	mut env: FunctionEnvMut<WasmEnv>,
	data_ptr: Ptr,
	data_len: u32,
	key_ptr: Ptr,
	key_len: u32,
) -> FFIResult {
	let (Ok(data), Ok(key)) = (
		env.data().read_bytes(&env, data_ptr, data_len),
		env.data().read_bytes(&env, key_ptr, key_len),
	) else {
		return -1;
	};
	let mut mac = match Hmac::<Sha1>::new_from_slice(&key) {
		Ok(mac) => mac,
		Err(_) => return -1,
	};
	mac.update(&data);
	store_hex(&mut env, mac.finalize().into_bytes().as_slice())
}

/// HMAC-SHA256 of `data` keyed by `key`, lowercase hex.
pub fn hmac_sha256(
	mut env: FunctionEnvMut<WasmEnv>,
	data_ptr: Ptr,
	data_len: u32,
	key_ptr: Ptr,
	key_len: u32,
) -> FFIResult {
	let (Ok(data), Ok(key)) = (
		env.data().read_bytes(&env, data_ptr, data_len),
		env.data().read_bytes(&env, key_ptr, key_len),
	) else {
		return -1;
	};
	let mut mac = match Hmac::<Sha256>::new_from_slice(&key) {
		Ok(mac) => mac,
		Err(_) => return -1,
	};
	mac.update(&data);
	store_hex(&mut env, mac.finalize().into_bytes().as_slice())
}

fn store_hex(env: &mut FunctionEnvMut<WasmEnv>, bytes: &[u8]) -> FFIResult {
	env.data_mut().store.store(StoreItem::String(to_hex(bytes)))
}

fn to_hex(bytes: &[u8]) -> String {
	use core::fmt::Write;
	let mut out = String::with_capacity(bytes.len() * 2);
	for byte in bytes {
		let _ = write!(out, "{byte:02x}");
	}
	out
}