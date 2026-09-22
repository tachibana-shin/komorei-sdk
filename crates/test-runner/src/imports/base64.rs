use crate::{
	FFIResult, Ptr, WasmEnv,
	libs::StoreItem,
};
use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine;
use wasmer::FunctionEnvMut;

/// Encode raw bytes as standard (RFC 4648 §4) base64 with padding.
pub fn encode(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(data) = env.data().read_bytes(&env, ptr, len) else {
		return -1;
	};
	env.data_mut().store.store(StoreItem::String(STANDARD.encode(&data)))
}

/// Tolerant decode: standard padded/unpadded and URL-safe padded/unpadded.
pub fn decode(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(data) = env.data().read_string(&env, ptr, len) else {
		return -1;
	};
	let decoded = STANDARD
		.decode(&data)
		.or_else(|_| STANDARD_NO_PAD.decode(&data))
		.or_else(|_| URL_SAFE.decode(&data))
		.or_else(|_| URL_SAFE_NO_PAD.decode(&data));
	match decoded {
		Ok(bytes) => env.data_mut().store.store(StoreItem::Encoded(bytes)),
		Err(_) => -1,
	}
}