#![no_std]
use komorei::{
	alloc::{vec, String, Vec},
	imports::{canvas::*, defaults::defaults_get, net::Request},
	prelude::*,
	Anime, AnimePageResult, AnimeSeason, AnimeStatus, AnimeWithEpisode, CategoryLink, CheckFilter,
	CoverImageProcessor, DeepLinkHandler, DeepLinkResult, DynamicFilters, DynamicListings,
	DynamicSettings, Episode, Filter, FilterValue, HashMap, Home, HomeComponent, HomeLayout,
	ImageResponse, Listing, ListingProvider, MigrationHandler, MultiSelectFilter,
	NotificationHandler, RangeFilter, RangeLong, Result, SelectFilter, Setting, SortFilter, Source,
	StreamData, StreamInfo, StreamType, SubtitleInfo, TextFilter, ToggleSetting,
};

const PAGE_SIZE: i32 = 20;

// real, playable sample media (playback works out of the box)
const SAMPLE_HLS: &str = "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8";
const SAMPLE_MP4: &str =
	"https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
const SAMPLE_COVER: &str = "https://example.com/cover.png";

// to create a source, you need a struct that implements the Source trait
// the struct can contain properties that are initialized with the new() method
struct ExampleSource;

impl Source for ExampleSource {
	// this method is called once when the source is initialized
	// perform any necessary setup here
	fn new() -> Self {
		Self
	}

	// this method will be called first without a query when the search page is opened,
	// then when a search query is entered or filters are changed
	fn get_search_anime_list(
		&self,
		query: Option<String>,
		page: i32,
		_filters: Vec<FilterValue>,
	) -> Result<AnimePageResult> {
		let mut entries: Vec<Anime> = Vec::new();
		let start = (page - 1) * PAGE_SIZE + 1;
		for i in start..start + PAGE_SIZE {
			let title = format!("Anime {i}");
			if let Some(query) = query.as_ref() {
				if !title.contains(query) {
					continue;
				}
			}
			entries.push(Anime {
				key: format!("{i}"),
				source_id: String::from("en.example-source"),
				title,
				original_title: String::from("Original title"),
				cover: String::from(SAMPLE_COVER),
				episode_count: 12,
				current_episode: Some(String::from("Tập 12/12")),
				status: AnimeStatus::Ongoing,
				genres: vec![CategoryLink {
					name: String::from("Action"),
					filters: Vec::new(),
				}],
				..Default::default()
			})
		}
		Ok(AnimePageResult {
			entries,
			has_next_page: start < 40,
		})
	}

	// this method will be called when an anime page is opened
	fn get_anime_update(
		&self,
		mut anime: Anime,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Anime> {
		if needs_details {
			anime.description = Self::fetch_example_title();
			anime.banner = Some(String::from(SAMPLE_COVER));
			anime.status = AnimeStatus::Ongoing;
			anime.release_year = Some(CategoryLink {
				name: String::from("2024"),
				filters: Vec::new(),
			});
			anime.authors = vec![CategoryLink {
				name: String::from("Author"),
				filters: Vec::new(),
			}];
			anime.studio = Some(CategoryLink {
				name: String::from("Studio"),
				filters: Vec::new(),
			});
			anime.rating = Some(8.5);
			anime.rating_count = Some(1234);
			anime.views = 99999;
			anime.next_episode_air_info = Some(String::from("Tập 13 phát sóng 20:00 thứ 7"));
			anime.quality_tag = Some(String::from("FHD"));
			anime.seasons = vec![
				AnimeSeason::new(String::from("1"), String::from("Season 1")),
				AnimeSeason::new(String::from("2"), String::from("Season 2")),
			];
			anime.url = Some(String::from("https://example.com/anime/1"));
		}
		if needs_chapters {
			anime.episodes = Some(vec![
				Episode {
					key: String::from("8"),
					episode_number: String::from("8"),
					..Default::default()
				},
				Episode {
					key: String::from("7"),
					episode_number: String::from("7"),
					title: Some(String::from("Title")),
					thumbnail: Some(String::from(SAMPLE_COVER)),
					quality: Some(String::from("1080p FHD")),
					..Default::default()
				},
				Episode {
					key: String::from("6"),
					episode_number: String::from("6"),
					title: Some(String::from("Title")),
					date_uploaded: Some(1692318525),
					..Default::default()
				},
				Episode {
					key: String::from("5"),
					episode_number: String::from("5"),
					..Default::default()
				},
				Episode {
					key: String::from("4"),
					episode_number: String::from("4"),
					..Default::default()
				},
				Episode {
					key: String::from("3"),
					episode_number: String::from("3"),
					..Default::default()
				},
				Episode {
					key: String::from("2"),
					episode_number: String::from("2"),
					..Default::default()
				},
				Episode {
					key: String::from("1"),
					episode_number: String::from("1"),
					..Default::default()
				},
			]);
		}
		Ok(anime)
	}

	// returns the playable streams (servers) for a given episode
	fn get_stream_list(&self, _anime: Anime, _episode: Episode) -> Result<Vec<StreamInfo>> {
		Ok(vec![
			StreamInfo {
				key: String::from("mux"),
				name: String::from("Server 1"),
				quality: String::from("1080p"),
			},
			StreamInfo {
				key: String::from("mp4"),
				name: String::from("Server 2"),
				quality: String::from("720p"),
			},
		])
	}

	// resolves the stream data (media url, headers, subtitles) for a given stream
	fn get_stream(&self, _anime: Anime, _episode: Episode, stream: StreamInfo) -> Result<StreamData> {
		// return `is_content: true` with a directly-playable url.
		// if a source returns `is_content: false`, the app resolves (fetches) the
		// url itself before playback.
		match stream.key.as_str() {
			"mp4" => Ok(StreamData {
				url: String::from(SAMPLE_MP4),
				stream_type: StreamType::MP4,
				is_content: true,
				headers: {
					let mut headers = HashMap::new();
					headers.insert(String::from("User-Agent"), String::from("Komorei/1.0"));
					headers
				},
				subtitles: Vec::new(),
				intro: None,
				outro: None,
			}),
			_ => Ok(StreamData {
				url: String::from(SAMPLE_HLS),
				stream_type: StreamType::HLS, // auto-detected by the media source factory
				is_content: true,
				headers: {
					let mut headers = HashMap::new();
					headers.insert(String::from("User-Agent"), String::from("Komorei/1.0"));
					headers
				},
				// sample subtitle track. replace with a real track url in a real source.
				subtitles: vec![SubtitleInfo {
					url: String::from("https://example.com/subs/vi.vtt"),
					language: String::from("vi"),
					label: Some(String::from("Tiếng Việt")),
					headers: HashMap::new(),
				}],
				// marks the opening segment on the progress bar & enables the skip button
				intro: Some(RangeLong {
					start_ms: 0,
					end_ms: 90_000,
				}),
				outro: Some(RangeLong {
					start_ms: 1_500_000,
					end_ms: 1_590_000,
				}),
			}),
		}
	}
}

impl ExampleSource {
	// fetches a page over the network and extracts the first <h1> via the
	// html parser — demonstrates the net -> html pipeline of the SDK
	fn fetch_example_title() -> Option<String> {
		Request::get("https://example.com")
			.ok()?
			.html()
			.ok()?
			.select_first("h1")?
			.text()
	}
}

// if your source provides any listings (static, dynamic, or in home components), this trait must be implemented
// this should probably be most sources
impl ListingProvider for ExampleSource {
	// this method will be called when a listing or a home section with an associated listing is opened
	fn get_anime_list(&self, listing: Listing, _page: i32) -> Result<AnimePageResult> {
		if listing.id == "test" {
			bail!("Not supported");
		}
		Ok(AnimePageResult {
			entries: vec![Anime {
				key: String::from("1"),
				source_id: String::from("en.example-source"),
				title: String::from("Anime 1"),
				cover: String::from(SAMPLE_COVER),
				..Default::default()
			}],
			has_next_page: false,
		})
	}
}

// use the home trait to implement a home page for a source
// where possible, try to replicate the associated web page's layout
impl Home for ExampleSource {
	fn get_home(&self) -> Result<HomeLayout> {
		let entries = self
			.get_search_anime_list(None, 1, Vec::new())?
			.entries;
		let episode = Episode {
			key: String::from("1"),
			episode_number: String::from("1"),
			title: Some(String::from("Episode")),
			date_uploaded: Some(1692318525),
			..Default::default()
		};
		let anime_episodes = entries
			.iter()
			.map(|anime| AnimeWithEpisode {
				anime: anime.clone(),
				episode: episode.clone(),
			})
			.take(3)
			.collect::<Vec<_>>();
		Ok(HomeLayout {
			components: vec![
				HomeComponent {
					title: Some(String::from("Big Scroller")),
					subtitle: None,
					value: komorei::HomeComponentValue::BigScroller {
						entries: entries.clone(),
						auto_scroll_interval: Some(10.0),
					},
				},
				HomeComponent {
					title: Some(String::from("Anime Episode List")),
					subtitle: None,
					value: komorei::HomeComponentValue::AnimeEpisodeList {
						page_size: None,
						entries: anime_episodes,
						listing: None,
					},
				},
				HomeComponent {
					title: Some(String::from("Anime List")),
					subtitle: None,
					value: komorei::HomeComponentValue::AnimeList {
						ranking: false,
						page_size: None,
						entries: entries.iter().take(2).cloned().map(|m| m.into()).collect(),
						listing: None,
					},
				},
				HomeComponent {
					title: Some(String::from("Anime List (Paged, Ranking)")),
					subtitle: None,
					value: komorei::HomeComponentValue::AnimeList {
						ranking: true,
						page_size: Some(3),
						entries: entries.iter().take(8).cloned().map(|m| m.into()).collect(),
						listing: None,
					},
				},
				HomeComponent {
					title: Some(String::from("Scroller")),
					subtitle: None,
					value: komorei::HomeComponentValue::Scroller {
						entries: entries.clone().into_iter().map(|m| m.into()).collect(),
						listing: None,
					},
				},
				HomeComponent {
					title: Some("Filters".into()),
					subtitle: None,
					value: komorei::HomeComponentValue::Filters(vec![
						komorei::FilterItem::from(String::from("Action")),
						"Adventure".into(),
						"Fantasy".into(),
						"Horror".into(),
						"Slice of Life".into(),
						"Magic".into(),
						"Adaptation".into(),
					]),
				},
				HomeComponent {
					title: Some(String::from("Links")),
					subtitle: None,
					value: komorei::HomeComponentValue::Links(vec![
						komorei::Link {
							title: String::from("Website Link"),
							value: Some(komorei::LinkValue::Url(String::from(
								"https://example.com",
							))),
							..Default::default()
						},
						komorei::Link {
							title: String::from("Anime Link"),
							value: Some(komorei::LinkValue::Anime(
								entries.first().unwrap().clone(),
							)),
							..Default::default()
						},
						komorei::Link {
							title: String::from("Listing Link"),
							value: Some(komorei::LinkValue::Listing(Listing {
								id: String::from("listing"),
								name: String::from("Listing"),
								kind: komorei::ListingKind::List,
							})),
							..Default::default()
						},
					]),
				},
			],
		})
	}
}

// if your source changes filters frequently or only has some filters available conditionally, use the DynamicFilters trait
// where possible, static filters are preferred
impl DynamicFilters for ExampleSource {
	fn get_dynamic_filters(&self) -> Result<Vec<Filter>> {
		Ok(vec![
			TextFilter {
				id: "text".into(),
				title: Some("Text".into()),
				placeholder: Some("Search".into()),
				..Default::default()
			}
			.into(),
			SortFilter {
				id: "sort".into(),
				title: Some("Sort".into()),
				can_ascend: true,
				options: vec!["Popular".into(), "Recent".into()],
				..Default::default()
			}
			.into(),
			CheckFilter {
				id: "check".into(),
				title: Some("Check".into()),
				can_exclude: true,
				..Default::default()
			}
			.into(),
			SelectFilter {
				id: "select".into(),
				title: Some("Select".into()),
				uses_tag_style: true,
				options: vec!["One".into(), "Two".into()],
				..Default::default()
			}
			.into(),
			MultiSelectFilter {
				id: "mselect".into(),
				title: Some("Multi-Select".into()),
				can_exclude: true,
				uses_tag_style: false,
				options: vec!["One".into(), "Two".into()],
				..Default::default()
			}
			.into(),
			Filter::note("Testing note"),
			RangeFilter {
				id: "range".into(),
				title: Some("Range".into()),
				min: Some(0.0),
				max: Some(100.0),
				decimal: true,
				..Default::default()
			}
			.into(),
		])
	}
}

// if you need to serve settings dynamically, use the DynamicSettings trait
// again, this shouldn't be used for static settings
impl DynamicSettings for ExampleSource {
	fn get_dynamic_settings(&self) -> Result<Vec<Setting>> {
		let toggle_value = defaults_get::<bool>("setting");
		let mut settings = vec![ToggleSetting {
			key: "setting".into(),
			title: "Toggle".into(),
			notification: Some("test".into()),
			refreshes: Some(vec!["settings".into()]),
			..Default::default()
		}
		.into()];
		if let Some(value) = toggle_value {
			if value {
				settings.push(
					ToggleSetting {
						key: "setting2".into(),
						title: "Toggle 2".into(),
						..Default::default()
					}
					.into(),
				);
			}
		}
		Ok(settings)
	}
}

// if you need to serve listings dynamically, use the DynamicListings trait
// again, this shouldn't be used for static listings
// for example, you could fetch listings from an API, or show one if a certain setting is enabled
impl DynamicListings for ExampleSource {
	fn get_dynamic_listings(&self) -> Result<Vec<Listing>> {
		Ok(vec![Listing {
			id: String::from("listing"),
			name: String::from("Listing"),
			kind: komorei::ListingKind::List,
		}])
	}
}

// if you need to perform any actions when settings change, use the NotificationHandler trait
// for example, you could update different defaults values
impl NotificationHandler for ExampleSource {
	fn handle_notification(&self, key: String) {
		println!("Notification: {key}");
	}
}

// you can process response data from cover images, and use canvas apis to draw your own images
// use the CoverImageProcessor trait
impl CoverImageProcessor for ExampleSource {
	fn process_cover_image(&self, _response: ImageResponse) -> Result<ImageRef> {
		let mut canvas = Canvas::new(200., 300.);
		canvas.fill(&Path::rect(&Rect::new(0., 0., 200., 300.)), &Color::white());
		canvas.draw_text(
			"Cover",
			32.,
			&Point::new(60., 140.),
			&Font::system(Default::default()),
			&Color::red(),
		);
		Ok(canvas.get_image())
	}
}

// it's recommended for all sources to implement the DeepLinkHandler trait
// the url that is passed in will have the base of any of the source's urls
// the source should determine if the url is a link to an anime, an episode, or a listing page,
// then return the appropriate DeepLinkResult to handle it.
impl DeepLinkHandler for ExampleSource {
	fn handle_deep_link(&self, _url: String) -> Result<Option<DeepLinkResult>> {
		Ok(Some(DeepLinkResult::Anime {
			key: String::from("anime_key"),
		}))
	}
}

// if your source is changing the way it formats anime and episode keys,
// implement the MigrationHandler trait to automatically handle the migration of existing data.
// this should be paired with the breakingChangeVersion inside the source configuration.
// if this trait isn't implemented, the app will default to showing the manual migration view.
impl MigrationHandler for ExampleSource {
	fn handle_anime_migration(&self, key: String) -> Result<String> {
		// example: add leading slash
		if key.starts_with('/') {
			Ok(key)
		} else {
			Ok(format!("/{key}"))
		}
	}

	fn handle_episode_migration(
		&self,
		_anime_key: String,
		episode_key: String,
	) -> Result<String> {
		// example: keep episode key as-is
		Ok(episode_key)
	}
}

// the register_source! macro generates the necessary wasm functions for the app
register_source!(
	ExampleSource,
	// after the name of the source struct, list all the extra traits it implements
	ListingProvider,
	Home,
	DynamicFilters,
	DynamicSettings,
	DynamicListings,
	NotificationHandler,
	CoverImageProcessor,
	DeepLinkHandler,
	MigrationHandler
);

// you can also implement tests via our custom test runner!
#[cfg(test)]
mod test {
	use super::*;
	use komorei_test::komorei_test;

	// all tests need to be annotated with the #[komorei_test] attribute instead of #[test]
	#[komorei_test]
	fn test_request() {
		let title = ExampleSource::fetch_example_title();
		println!("{:?}", title); // if the test fails (or you pass --nocapture), you can see this in the log,
		assert_eq!(title.as_deref(), Some("Example Domain"));
	}

	#[komorei_test]
	fn test_js_execution() {
		// most komorei imports you'd want to use should also work
		use komorei::imports::js::JsContext;
		let context = JsContext::new();
		let result = context.eval("1 + 2");
		assert_eq!(result, Ok(String::from("3")));
	}

	#[komorei_test]
	fn test_stream_list() {
		let anime = Anime {
			key: String::from("1"),
			source_id: String::from("en.example-source"),
			title: String::from("Anime 1"),
			cover: String::from(SAMPLE_COVER),
			..Default::default()
		};
		let episode = Episode {
			key: String::from("1"),
			episode_number: String::from("1"),
			..Default::default()
		};
		let streams = ExampleSource.get_stream_list(anime, episode).unwrap();
		assert_eq!(streams.len(), 2);
		let data = ExampleSource.get_stream(
			Anime::default(),
			Episode::default(),
			streams[0].clone(),
		);
		assert!(data.is_ok());
		let err = komorei::KomoreiError::Message(String::from("expected"));
		assert_eq!(err.error_code(), -1);
	}
}