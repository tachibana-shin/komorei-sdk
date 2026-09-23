use super::{
	Anime, AnimePageResult, Episode, Filter, FilterValue, HashMap, HomeLayout, Listing, Setting,
	StreamData, StreamInfo,
};
use crate::alloc::{String, Vec};
use crate::imports::canvas::ImageRef;
use serde::{Deserialize, Serialize, ser::SerializeStruct};

pub use crate::imports::error::{KomoreiError, Result};

/// The required functions a Komorei source must implement.
pub trait Source {
	/// Called to initialize a source.
	///
	/// If a source requires any setup before other functions are called, it should happen here.
	fn new() -> Self;

	/// Returns the anime for a search query with filters.
	fn get_search_anime_list(
		&self,
		query: Option<String>,
		page: i32,
		filters: Vec<FilterValue>,
	) -> Result<AnimePageResult>;

	/// Updates a given anime with new details and chapters (episodes), as requested.
	fn get_anime_update(
		&self,
		anime: Anime,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Anime>;

	/// Returns the playable streams (servers) for a given anime episode.
	fn get_stream_list(&self, anime: Anime, episode: Episode) -> Result<Vec<StreamInfo>>;

	/// Resolves the stream data (media url, headers, subtitles) for a given stream.
	fn get_stream(&self, anime: Anime, episode: Episode, stream: StreamInfo) -> Result<StreamData>;
}

/// A source that rewrites media request urls before they are fetched.
///
/// The media engine calls this for every request it performs (playlist,
/// segments, chunks, ...) and fetches the returned url instead. Sources use it
/// to decorate urls with session tokens, referers, or server-side signatures
/// that change per request. Return the url unchanged to leave it untouched.
pub trait SegmentUrlInterceptor: Source {
	fn intercept_segment_url(&self, stream_data: Option<&StreamData>, url: String) -> String;
}

/// A source that transforms media response data after it is fetched.
///
/// The media engine calls this with the raw bytes of every response body it
/// fetches before passing them to the decoder. Sources use it to de-obfuscate
/// or decrypt HLS/VOD segments, or rewrite a mangled playlist. Return the data
/// unchanged to leave it untouched.
pub trait SegmentDataInterceptor: Source {
	fn intercept_segment_data(
		&self,
		stream_data: Option<&StreamData>,
		url: String,
		data: &[u8],
	) -> Vec<u8>;
}

/// A source that provides listings.
pub trait ListingProvider: Source {
	/// Returns the anime for the provided listing.
	fn get_anime_list(&self, listing: Listing, page: i32) -> Result<AnimePageResult>;
}

/// A source that provides a home layout.
pub trait Home: Source {
	fn get_home(&self) -> Result<HomeLayout>;
}

/// A source that provides dynamic listings.
pub trait DynamicListings: Source {
	fn get_dynamic_listings(&self) -> Result<Vec<Listing>>;
}

/// A source that provides dynamic filters.
pub trait DynamicFilters: Source {
	fn get_dynamic_filters(&self) -> Result<Vec<Filter>>;
}

/// A source that provides dynamic settings.
pub trait DynamicSettings: Source {
	fn get_dynamic_settings(&self) -> Result<Vec<Setting>>;
}

/// A source that processes cover image data after being fetched.
pub trait CoverImageProcessor: Source {
	fn process_cover_image(&self, response: ImageResponse) -> Result<ImageRef>;
}

/// A source that provides a programmatic base url.
///
/// The use of this trait is discouraged in favor of providing the source url statically.
pub trait BaseUrlProvider: Source {
	fn get_base_url(&self) -> Result<String>;
}

/// A source that handles notification callbacks.
///
/// Notifications can be sent on source setting changes.
pub trait NotificationHandler: Source {
	fn handle_notification(&self, notification: String);
}

/// A source that handles deep links.
///
/// If a url that is contained in one of the source's provided base urls is opened
/// in Komorei, it will be sent to the given source to handle.
pub trait DeepLinkHandler: Source {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>>;
}

/// A source that provides "related / recommended" anime for a given title.
///
/// The app shows the result in the "Có thể bạn sẽ thích" (you may also like)
/// section, re-queried whenever a different anime is opened. A source with no
/// dedicated recommendations endpoint simply does NOT implement this trait —
/// the app then falls back to a search (`get_search_anime_list`) by the anime's
/// FIRST genre tag, so the section still fills with same-genre titles.
pub trait RecommendationsHandler: Source {
	fn get_recommended_anime(&self, anime: Anime) -> Result<AnimePageResult>;
}

/// A source that handles basic login with username and password.
///
/// This function should return true if the login was successful.
pub trait BasicLoginHandler: Source {
	fn handle_basic_login(&self, key: String, username: String, password: String) -> Result<bool>;
}

/// A source that handles custom webview login.
///
/// This function will be called whenever cookies are updated, and should return true if the login was successful.
pub trait WebLoginHandler: Source {
	fn handle_web_login(&self, key: String, cookies: HashMap<String, String>) -> Result<bool>;
}

/// A source that handles key migration.
///
/// These functions are called with all of a user's local anime and episode keys to
/// migrate them after an update. They should return the new key to replace the old one.
pub trait MigrationHandler: Source {
	fn handle_anime_migration(&self, key: String) -> Result<String>;
	fn handle_episode_migration(&self, anime_key: String, episode_key: String) -> Result<String>;
}

/// A result of a deep link handling.
#[derive(Debug, Clone, PartialEq)]
pub enum DeepLinkResult {
	Anime { key: String },
	Episode { anime_key: String, key: String },
	Listing(Listing),
}

impl Serialize for DeepLinkResult {
	fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("DeepLinkResult", 3)?;
		match self {
			DeepLinkResult::Anime { key } => {
				state.serialize_field("anime_key", &Some(key))?;
				state.serialize_field("episode_key", &Option::<String>::None)?;
				state.serialize_field("listing", &Option::<Listing>::None)?;
			}
			DeepLinkResult::Episode { anime_key, key } => {
				state.serialize_field("anime_key", &Some(anime_key))?;
				state.serialize_field("episode_key", &Some(key))?;
				state.serialize_field("listing", &Option::<Listing>::None)?;
			}
			DeepLinkResult::Listing(listing) => {
				state.serialize_field("anime_key", &Option::<String>::None)?;
				state.serialize_field("episode_key", &Option::<String>::None)?;
				state.serialize_field("listing", &Some(listing))?;
			}
		}
		state.end()
	}
}

/// The details of a HTTP request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageRequest {
	pub url: Option<String>,
	pub headers: HashMap<String, String>,
}

/// A response from a network image request.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResponse {
	/// The HTTP status code.
	pub code: u16,
	/// The HTTP response headers.
	pub headers: HashMap<String, String>,
	/// The HTTP request details.
	pub request: ImageRequest,
	/// A reference to image data.
	pub image: ImageRef,
}