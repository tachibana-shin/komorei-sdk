use super::{Anime, Episode, FilterValue, Listing};
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::{string::String, vec::Vec};

/// A partial result for the home page.
///
/// This should only be used with [send_partial_result](crate::imports::std::send_partial_result)
/// in the [get_home](super::Home::get_home) function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HomePartialResult {
	Layout(HomeLayout),
	Component(HomeComponent),
}

/// A home layout for a source.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomeLayout {
	/// The components of the layout.
	pub components: Vec<HomeComponent>,
}

/// A component for a home layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeComponent {
	/// The title of the component.
	pub title: Option<String>,
	/// The subtitle of the component.
	pub subtitle: Option<String>,
	/// The component value.
	pub value: HomeComponentValue,
}

impl Default for HomeComponent {
	fn default() -> Self {
		Self {
			title: None,
			subtitle: None,
			value: HomeComponentValue::empty_scroller(),
		}
	}
}

/// The value of a component for a home layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HomeComponentValue {
	/// A horizontal scroller of images.
	///
	/// Only the image urls and values of the provided links are used.
	ImageScroller {
		links: Vec<Link>,
		auto_scroll_interval: Option<f32>,
		width: Option<i32>,
		height: Option<i32>,
	},
	/// A large scroller of anime.
	///
	/// This component displays the title, authors, cover image, description,
	/// content rating and tags of the provided anime entries.
	BigScroller {
		entries: Vec<Anime>,
		auto_scroll_interval: Option<f32>,
	},
	/// A small scroller of anime.
	///
	/// The subtitles of the provided links are not used.
	Scroller {
		entries: Vec<Link>,
		listing: Option<Listing>,
	},
	/// A list of anime.
	AnimeList {
		/// If the list should be displayed with ranking numbers.
		ranking: bool,
		page_size: Option<i32>,
		entries: Vec<Link>,
		listing: Option<Listing>,
	},
	/// A list of anime episodes.
	///
	/// The relative time to the episode's date uploaded is displayed if provided.
	AnimeEpisodeList {
		page_size: Option<i32>,
		entries: Vec<AnimeWithEpisode>,
		listing: Option<Listing>,
	},
	/// A collection of links to filtered listings.
	Filters(Vec<FilterItem>),
	/// A list of links.
	///
	/// Only the link title and values are used.
	Links(Vec<Link>),
}

impl HomeComponentValue {
	/// Creates an empty image scroller component.
	pub fn empty_image_scroller() -> Self {
		Self::ImageScroller {
			links: Vec::new(),
			auto_scroll_interval: None,
			width: None,
			height: None,
		}
	}

	/// Creates an empty big scroller component.
	pub fn empty_big_scroller() -> Self {
		Self::BigScroller {
			entries: Vec::new(),
			auto_scroll_interval: None,
		}
	}

	/// Creates an empty scroller component.
	pub fn empty_scroller() -> Self {
		Self::Scroller {
			entries: Vec::new(),
			listing: None,
		}
	}

	/// Creates an empty anime list component.
	pub fn empty_anime_list() -> Self {
		Self::AnimeList {
			ranking: false,
			page_size: None,
			entries: Vec::new(),
			listing: None,
		}
	}

	/// Creates an empty anime episode list component.
	pub fn empty_anime_episode_list() -> Self {
		Self::AnimeEpisodeList {
			page_size: None,
			entries: Vec::new(),
			listing: None,
		}
	}

	/// Creates an empty filters component.
	pub fn empty_filters() -> Self {
		Self::Filters(Vec::new())
	}

	/// Creates an empty links component.
	pub fn empty_links() -> Self {
		Self::Links(Vec::new())
	}
}

/// A paired anime and episode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimeWithEpisode {
	pub anime: Anime,
	pub episode: Episode,
}

/// A link to a listing that uses the provided filters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterItem {
	pub title: String,
	pub values: Option<Vec<FilterValue>>,
}

impl From<String> for FilterItem {
	fn from(title: String) -> Self {
		Self {
			title,
			values: None,
		}
	}
}

impl From<&str> for FilterItem {
	fn from(title: &str) -> Self {
		Self {
			title: String::from(title),
			values: None,
		}
	}
}

/// A link used in home components.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
	pub title: String,
	pub subtitle: Option<String>,
	pub image_url: Option<String>,
	pub value: Option<LinkValue>,
}

/// A link value that can be opened by the Komorei app.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkValue {
	Url(String),
	Listing(Listing),
	Anime(Anime),
}

impl Default for LinkValue {
	fn default() -> Self {
		Self::Url(String::new())
	}
}

impl From<Anime> for Link {
	fn from(value: Anime) -> Self {
		let authors = value
			.authors
			.iter()
			.map(|a| a.name.clone())
			.collect::<Vec<String>>()
			.join(", ");
		Link {
			title: value.title.clone(),
			subtitle: if authors.is_empty() {
				value.description.clone()
			} else {
				Some(authors)
			},
			image_url: Some(value.cover.clone()),
			value: Some(LinkValue::Anime(value)),
		}
	}
}