//! Streamable video data structures shared between Komorei and sources.

use super::HashMap;
use crate::alloc::{String, Vec};
use serde::{Deserialize, Serialize};

/// The streaming format of a media source.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StreamType {
	/// HTTP Live Streaming (m3u8 playlist).
	#[default]
	HLS,
	/// Direct MP4 file.
	MP4,
	/// DASH (mpd manifest).
	DASH,
	/// Any other resolvable format.
	OTHER,
}

/// A playable stream (server) for an episode.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamInfo {
	/// Unique identifier for the stream.
	pub key: String,
	/// Display name of the stream/server.
	pub name: String,
	/// Quality label of the stream.
	pub quality: String,
}

/// A millisecond time range — used to mark an episode's opening (intro) or
/// ending (outro) segment on the progress bar and for skip buttons.
#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RangeLong {
	/// Start of the range, in milliseconds.
	pub start_ms: i64,
	/// End of the range, in milliseconds.
	pub end_ms: i64,
}

/// Resolved stream data used by the media engine.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamData {
	/// The media url, or the raw media content itself when `is_content` is true.
	pub url: String,
	/// The streaming format.
	pub stream_type: StreamType,
	/// Whether `url` holds the raw media content itself — e.g. an HLS playlist
	/// text starting with "#EXTM3U...". `false` means the url must be resolved
	/// (fetched) before playback.
	pub is_content: bool,
	/// Extra headers (Referer, User-Agent, ...) applied to every media sub-request.
	pub headers: HashMap<String, String>,
	/// Optional subtitle tracks.
	pub subtitles: Vec<SubtitleInfo>,
	/// Time range of the opening (intro) segment.
	pub intro: Option<RangeLong>,
	/// Time range of the ending (outro) segment.
	pub outro: Option<RangeLong>,
}

/// A subtitle track for a stream.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubtitleInfo {
	/// Url of the subtitle file.
	pub url: String,
	/// Language of the subtitle (e.g. "vi").
	pub language: String,
	/// Display label of the subtitle.
	pub label: Option<String>,
	/// Extra headers for fetching this subtitle.
	pub headers: HashMap<String, String>,
}