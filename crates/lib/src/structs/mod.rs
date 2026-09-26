//! Serializable data structures that are sent between Komorei and sources.

use super::alloc::{String, Vec};
use serde::{Deserialize, Serialize};

pub use hashbrown::HashMap;

pub mod canvas;
mod filter;
mod home;
mod setting;
mod stream;

pub use filter::*;
pub use home::*;
pub use setting::*;
pub use stream::*;

#[cfg(feature = "imports")]
mod source;

#[cfg(feature = "imports")]
pub use source::*;

/// The publishing status of an anime.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AnimeStatus {
	#[default]
	Unknown = 0,
	Ongoing,
	Completed,
	Cancelled,
	Hiatus,
}

/// The content rating of an anime.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ContentRating {
	#[default]
	Unknown = 0,
	Safe,
	Suggestive,
	NSFW,
}

/// The preferred update strategy for an anime.
///
/// Titles marked as `Always` will be included in library refreshes by default,
/// while `Never` will be excluded. Useful for titles that are known to be fully
/// completed or have a static episode list that won't change after the initial fetch.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum UpdateStrategy {
	#[default]
	Always,
	Never,
}

/// An interactive metadata link used for filtering (genre, studio, author, ...).
///
/// Selecting a link in the app applies its filters to a search.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoryLink {
	/// Display name of the link.
	pub name: String,
	/// Filters applied when the link is selected, mapped to [FilterValue]s.
	pub filters: Vec<FilterValue>,
}

/// A specific season or part of a franchise.
///
/// Seasons are often separate anime ids in many source extensions.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimeSeason {
	/// The anime id of the season.
	pub anime_id: String,
	/// Title of the season.
	pub title: String,
	/// Unique identity. Defaults to `anime_id`; virtual seasons may override it.
	pub id: String,
}

impl AnimeSeason {
	/// Create a new season with the given anime id and title.
	pub fn new(anime_id: String, title: String) -> Self {
		Self {
			anime_id: anime_id.clone(),
			title,
			id: anime_id,
		}
	}
}

/// An anime, series, or other type of content for Komorei to watch.
///
/// Usually fetched in two stages: Lite (listing) and Full (details + episodes).
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Anime {
	/// Unique identifier for the anime.
	pub key: String,
	/// Identifier of the source this anime belongs to.
	pub source_id: String,
	/// Title of the anime.
	pub title: String,
	/// Original (raw) title of the anime.
	pub original_title: String,
	/// Link to the anime cover (poster) image.
	pub cover: String,
	/// Link to the anime banner image.
	pub banner: Option<String>,
	/// Description of the anime.
	pub description: Option<String>,
	/// Total number of episodes.
	pub episode_count: i32,
	/// Current episode label, e.g. "Tập 12/12" or "HD Vietsub".
	pub current_episode: Option<String>,
	/// Rating of the anime (0 to 10).
	pub rating: Option<f32>,
	/// Number of ratings.
	pub rating_count: Option<i32>,
	/// Publishing status of the anime.
	pub status: AnimeStatus,
	/// Year the anime was released.
	pub release_year: Option<CategoryLink>,
	/// List of genres.
	pub genres: Vec<CategoryLink>,
	/// List of authors.
	pub authors: Vec<CategoryLink>,
	/// Studio that produced the anime.
	pub studio: Option<CategoryLink>,
	/// The season this anime belongs to.
	pub season_of: Option<CategoryLink>,
	/// List of countries.
	pub countries: Vec<CategoryLink>,
	/// Whether the anime is featured (e.g. on the home banner).
	pub is_featured: bool,
	/// Number of views.
	pub views: i32,
	/// Info on the next episode airing, e.g. "Tập 13 phát sóng 20:00 thứ 7".
	pub next_episode_air_info: Option<String>,
	/// Quality tag of the anime, e.g. "FHD".
	pub quality_tag: Option<String>,
	/// List of seasons.
	pub seasons: Vec<AnimeSeason>,
	/// List of episodes. Only populated when `needs_chapters` is set.
	pub episodes: Option<Vec<Episode>>,
	/// Link to the anime on the source website.
	pub url: Option<String>,
	/// Source-defined extras, keyed by a dotted name (e.g. `"avs.recommendations"`).
	///
	/// A place for data the site serves on a page the app does not model, so a
	/// source can hand over something it would otherwise have to re-request:
	/// AnimeVietsub's detail page carries its own "related" rail, and stashing
	/// the parsed keys here saves the app a second visit to a page it has just
	/// loaded.
	///
	/// A flat string map on purpose — it round-trips through the runner's ABI
	/// without a schema, so the app reads it opaquely and no source can break
	/// another by choosing a key. A source that stores structured data should
	/// encode it (JSON is the usual choice) rather than widen this type.
	#[serde(default)]
	pub extra: HashMap<String, String>,
}

impl Anime {
	/// Copy the values from another anime into this one.
	///
	/// Used to upgrade a Lite anime with full details and episodes.
	pub fn copy_from(&mut self, anime: Anime) {
		self.key = anime.key;
		self.source_id = anime.source_id;
		self.title = anime.title;
		self.original_title = anime.original_title;
		self.cover = anime.cover;
		if let Some(banner) = anime.banner {
			self.banner = Some(banner);
		}
		if let Some(description) = anime.description {
			self.description = Some(description);
		}
		if anime.episode_count != 0 {
			self.episode_count = anime.episode_count;
		}
		if let Some(current_episode) = anime.current_episode {
			self.current_episode = Some(current_episode);
		}
		if let Some(rating) = anime.rating {
			self.rating = Some(rating);
		}
		if let Some(rating_count) = anime.rating_count {
			self.rating_count = Some(rating_count);
		}
		self.status = anime.status;
		if let Some(release_year) = anime.release_year {
			self.release_year = Some(release_year);
		}
		if !anime.genres.is_empty() {
			self.genres = anime.genres;
		}
		if !anime.authors.is_empty() {
			self.authors = anime.authors;
		}
		if let Some(studio) = anime.studio {
			self.studio = Some(studio);
		}
		if let Some(season_of) = anime.season_of {
			self.season_of = Some(season_of);
		}
		if !anime.countries.is_empty() {
			self.countries = anime.countries;
		}
		if anime.views != 0 {
			self.views = anime.views;
		}
		if let Some(next_episode_air_info) = anime.next_episode_air_info {
			self.next_episode_air_info = Some(next_episode_air_info);
		}
		if let Some(quality_tag) = anime.quality_tag {
			self.quality_tag = Some(quality_tag);
		}
		if !anime.seasons.is_empty() {
			self.seasons = anime.seasons;
		}
		if let Some(episodes) = anime.episodes {
			self.episodes = Some(episodes);
		}
		if let Some(url) = anime.url {
			self.url = Some(url);
		}
		// Merged rather than replaced: a Lite pass may already have cached an
		// extra the full pass knows nothing about (or the other way round), and
		// the full pass is the one that wins on a shared key.
		for (key, value) in anime.extra {
			self.extra.insert(key, value);
		}
	}
}

/// A page of anime entries.
#[derive(Default, Clone, Debug, PartialEq, Serialize)]
pub struct AnimePageResult {
	/// List of anime entries.
	pub entries: Vec<Anime>,
	/// Whether the next page is available or not.
	pub has_next_page: bool,
}

/// An episode of an anime.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Episode {
	/// Unique identifier for the episode.
	pub key: String,
	/// Episode number, as displayed by the source (e.g. "1", "12.5", "HD").
	pub episode_number: String,
	/// Title of the episode (excluding the episode number).
	pub title: Option<String>,
	/// Date the episode was uploaded (unix timestamp).
	pub date_uploaded: Option<i64>,
	/// Optional thumbnail image url for the episode.
	pub thumbnail: Option<String>,
	/// Quality of the episode, e.g. "1080p FHD".
	pub quality: Option<String>,
	/// Duration of the episode in seconds (reference only; real duration comes from the stream).
	pub duration_seconds: Option<i64>,
	/// Link to the episode on the source website.
	pub url: Option<String>,
	/// Language of the episode.
	pub language: Option<String>,
	/// Boolean indicating if the episode is locked.
	pub locked: bool,
}

/// The display type of a listing.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ListingKind {
	#[default]
	Default,
	List,
}

/// A listing of anime.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
	/// Unique identifier for the listing.
	pub id: String,
	/// Title of the listing.
	pub name: String,
	/// Type of listing.
	pub kind: ListingKind,
}