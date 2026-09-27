use crate::{FFIResult, Ptr, WasmEnv, libs::StoreItem};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use wasmer::FunctionEnvMut;

/// Encode raw bytes as standard (RFC 4648 §4) base64 with padding.
pub fn encode(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(data) = env.data().read_bytes(&env, ptr, len) else {
		return -1;
	};
	env.data_mut()
		.store
		.store(StoreItem::String(STANDARD.encode(&data)))
}

/// Decode, following the WHATWG *forgiving-base64 decode* algorithm — the one a
/// browser's `atob` implements, and the reference every site-side grab script is
/// written against.
///
/// **This must stay identical to the app runner's implementation** in
/// `runner/src/imports/base64.rs`. The two hosts exist so a source's tests can
/// run without a device; if they disagree, the tests describe an environment the
/// app never runs in, and every base64 edge case in a source goes unproven.
pub fn decode(mut env: FunctionEnvMut<WasmEnv>, ptr: Ptr, len: u32) -> FFIResult {
	let Ok(data) = env.data().read_string(&env, ptr, len) else {
		return -1;
	};
	let decoded = forgiving_decode(&data, false).or_else(|()| forgiving_decode(&data, true));
	match decoded {
		Ok(bytes) => env.data_mut().store.store(StoreItem::Encoded(bytes)),
		Err(_) => -1,
	}
}

type ForgivingResult = Result<Vec<u8>, ()>;

/// WHATWG *forgiving-base64 decode* over the standard alphabet, or over the
/// URL-safe one when [url_safe] is set.
///
/// Four ways this is more permissive than a strict decoder, all matching `atob`:
/// ASCII whitespace is stripped rather than rejected, missing padding is fine,
/// surplus padding is fine, and non-zero trailing bits are discarded rather than
/// treated as corruption. What `atob` still rejects — a length of `4n + 1`, a
/// `=` away from the end, and any character outside the alphabet — is rejected
/// here too.
fn forgiving_decode(input: &str, url_safe: bool) -> ForgivingResult {
	let cleaned: String = input
		.chars()
		.filter(|c| !matches!(*c as u32, 0x09 | 0x0A | 0x0C | 0x0D | 0x20))
		.collect();

	let body = cleaned.trim_end_matches('=');

	if body.contains('=') {
		return Err(());
	}

	let mut out: Vec<u8> = Vec::with_capacity(body.len() * 3 / 4 + 3);
	let mut buffer: u32 = 0;
	let mut pending: u32 = 0;
	let mut chars: usize = 0;

	for c in body.chars() {
		let value = match c {
			'A'..='Z' => c as u32 - 'A' as u32,
			'a'..='z' => c as u32 - 'a' as u32 + 26,
			'0'..='9' => c as u32 - '0' as u32 + 52,
			'+' if !url_safe => 62,
			'/' if !url_safe => 63,
			'-' if url_safe => 62,
			'_' if url_safe => 63,
			_ => return Err(()),
		};
		chars += 1;
		buffer = (buffer << 6) | value;
		pending += 6;
		if pending == 24 {
			out.push((buffer >> 16) as u8);
			out.push((buffer >> 8) as u8);
			out.push(buffer as u8);
			buffer = 0;
			pending = 0;
		}
	}

	if chars % 4 == 1 {
		return Err(());
	}

	match pending {
		12 => out.push((buffer >> 4) as u8),
		18 => {
			out.push((buffer >> 10) as u8);
			out.push((buffer >> 2) as u8);
		}
		_ => {}
	}

	Ok(out)
}

#[cfg(test)]
mod tests {
	use super::forgiving_decode;

	fn std(input: &str) -> Option<Vec<u8>> {
		forgiving_decode(input, false).ok()
	}

	#[test]
	fn matches_what_a_browser_atob_accepts() {
		// Canonical, missing padding, surplus padding, whitespace, and the
		// non-zero trailing bits `atob` discards instead of rejecting.
		assert_eq!(std("QQ=="), Some(b"A".to_vec()));
		assert_eq!(std("QQ"), Some(b"A".to_vec()));
		assert_eq!(std("QQ="), Some(b"A".to_vec()));
		assert_eq!(std("QQ==="), Some(b"A".to_vec()));
		assert_eq!(std("QR=="), Some(b"A".to_vec()));
		assert_eq!(std("QU\nJD"), Some(b"ABC".to_vec()));
		assert_eq!(std("Zm9vYmFy"), Some(b"foobar".to_vec()));
		// Still rejected.
		assert_eq!(std("Q"), None);
		assert_eq!(std("QU=JD"), None);
		assert_eq!(std("QU!JD"), None);
	}
}
