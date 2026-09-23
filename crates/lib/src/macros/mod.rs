/// Prints to Komorei logs.
///
/// # Examples
///
/// ```ignore
/// println!(); // prints just a newline
/// println!("hello there!");
/// println!("format {} arguments", "some");
/// let local_variable = "some";
/// println!("format {local_variable} arguments");
/// ```
#[macro_export]
macro_rules! println {
	() => {
		$crate::alloc::print("");
	};
	($($arg:tt)*) => {
		$crate::imports::std::print(&$crate::prelude::format!($($arg)*));
	};
}

/// Prints to Komorei logs if debug assertions are enabled.
#[macro_export]
macro_rules! debug {
	() => {
		#[cfg(debug_assertions)]
		{
			$crate::prelude::println!();
		}
	};
	($($arg:tt)*) => {
		#[cfg(debug_assertions)]
		{
			$crate::prelude::println!($($arg)*);
		}
	};
}

/// Constructs an error with a message.
///
/// This macro is equivalent to
/// <code>KomoreiError::message(format!($args\...))</code>.
#[macro_export]
macro_rules! error {
	($($arg:tt)*) => {
		$crate::KomoreiError::message($crate::prelude::format!($($arg)*))
	};
}

/// Returns early with an error.
///
/// This macro is equivalent to
/// <code>return Err(error!($args\...))</code>.
///
/// The surrounding function's or closure's return value is required to be
/// <code>Result&lt;_, [KomoreiError][crate::KomoreiError]&gt;</code>.
#[macro_export]
macro_rules! bail {
	($($arg:tt)*) => {
		return ::core::result::Result::Err($crate::error!($($arg)*));
	};
}

/// Registers a source for use with Komorei.
///
/// The first argument should be the struct that implements the Source trait, and the
/// following arguments should be all the additional traits that the source implements.
///
/// # Examples
///
/// ```ignore
/// struct TestSource;
///
/// impl Source for TestSource { /* ... */ }
/// impl Home for TestSource { /* ... */ }
///
/// // register TestSource with the extra Home trait
/// register_source!(TestSource, Home);
/// ```
#[macro_export]
macro_rules! register_source {
	($source_type:ty $(, $param:ident)*) => {
		static mut SOURCE: ::core::option::Option<$crate::alloc::Box<$source_type>> =
			::core::option::Option::None;

		fn __source() -> &'static mut $source_type {
			unsafe { SOURCE.as_deref_mut().unwrap() }
		}

		fn __handle_result<T: $crate::serde::Serialize>(
			result: ::core::result::Result<T, $crate::imports::error::KomoreiError>,
		) -> i32 {
			match &result {
				::core::result::Result::Ok(result) => {
					let mut bytes = $crate::postcard::to_allocvec(result).unwrap();

				 	bytes.splice(0..0, [0,0,0,0,0,0,0,0]);
					let len_bytes = (bytes.len() as i32).to_le_bytes();
					bytes[0..4].copy_from_slice(&len_bytes);
					let cap_bytes = (bytes.capacity() as i32).to_le_bytes();
					bytes[4..8].copy_from_slice(&cap_bytes);

					let ptr = bytes.as_ptr() as i32;
					::core::mem::forget(bytes);
					ptr
				}
				::core::result::Result::Err(err) => __handle_error(err),
			}
		}

		fn __handle_error(error: &$crate::imports::error::KomoreiError) -> i32 {
			$crate::prelude::println!("Error: {:?}", error);
			match error {
				$crate::imports::error::KomoreiError::Message(string) => {
					let mut buffer = (-1 as i32).to_le_bytes().to_vec();

					buffer.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
					buffer.extend_from_slice(&string.as_bytes());

					let cap_bytes = (buffer.capacity() as i32).to_le_bytes();
					buffer[4..8].copy_from_slice(&cap_bytes);
					let len_bytes = (buffer.len() as i32).to_le_bytes();
					buffer[8..12].copy_from_slice(&len_bytes);

					let ptr = buffer.as_ptr() as i32;
					::core::mem::forget(buffer);
					ptr
				}
				error => error.error_code(),
			}
		}

		// once rust supports exporting a wasm start function, we should use that instead
		#[unsafe(export_name = "start")]
		pub extern "C" fn __start() {
			unsafe {
				SOURCE =
					::core::option::Option::Some($crate::alloc::Box::new(<$source_type>::new()))
			};
		}

		#[unsafe(no_mangle)]
		#[unsafe(export_name = "free_result")]
		pub unsafe extern "C" fn __wasm_free_result(ptr: i32) {
			$crate::imports::std::free_result(ptr);
		}

		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_search_anime_list")]
		pub unsafe extern "C" fn __wasm_get_search_anime_list(
			query_descriptor: i32,
			page: i32,
			filters_descriptor: i32,
		) -> i32 {
			let query = $crate::imports::std::read_string(query_descriptor);
			let ::core::result::Result::Ok(filters) =
				$crate::imports::std::read::<$crate::alloc::Vec<$crate::FilterValue>>(filters_descriptor)
			else {
				return -1;
			};

			let result = __source().get_search_anime_list(query, page, filters);
			__handle_result(result)
		}

		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_anime_update")]
		pub unsafe extern "C" fn __wasm_get_anime_update(
			anime_descriptor: i32,
			needs_details: bool,
			needs_chapters: bool,
		) -> i32 {
			let ::core::result::Result::Ok(anime) =
				$crate::imports::std::read::<$crate::Anime>(anime_descriptor)
			else {
				return -1;
			};

			let result = __source().get_anime_update(anime, needs_details, needs_chapters);
			__handle_result(result)
		}

		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_stream_list")]
		pub unsafe extern "C" fn __wasm_get_stream_list(
			anime_descriptor: i32,
			episode_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(anime) =
				$crate::imports::std::read::<$crate::Anime>(anime_descriptor)
			else {
				return -1;
			};
			let ::core::result::Result::Ok(episode) =
				$crate::imports::std::read::<$crate::Episode>(episode_descriptor)
			else {
				return -2;
			};

			let result = __source().get_stream_list(anime, episode);
			__handle_result(result)
		}

		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_stream")]
		pub unsafe extern "C" fn __wasm_get_stream(
			anime_descriptor: i32,
			episode_descriptor: i32,
			stream_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(anime) =
				$crate::imports::std::read::<$crate::Anime>(anime_descriptor)
			else {
				return -1;
			};
			let ::core::result::Result::Ok(episode) =
				$crate::imports::std::read::<$crate::Episode>(episode_descriptor)
			else {
				return -2;
			};
			let ::core::result::Result::Ok(stream) =
				$crate::imports::std::read::<$crate::StreamInfo>(stream_descriptor)
			else {
				return -3;
			};

			let result = __source().get_stream(anime, episode, stream);
			__handle_result(result)
		}

		$(
			register_source!(@single $param);
		)*
	};

	(@single ListingProvider) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_anime_list")]
		pub unsafe extern "C" fn __wasm_get_anime_list(listing_descriptor: i32, page: i32) -> i32 {
			let ::core::result::Result::Ok(listing) =
				$crate::imports::std::read::<$crate::Listing>(listing_descriptor)
			else {
				return -1;
			};

			use $crate::ListingProvider;
			let result = __source().get_anime_list(listing, page);
			__handle_result(result)
		}
	};

	(@single Home) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_home")]
		pub unsafe extern "C" fn __wasm_get_home() -> i32 {
			use $crate::Home;
			let result = __source().get_home();
			__handle_result(result)
		}
	};

	(@single DynamicListings) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_listings")]
		pub unsafe extern "C" fn __wasm_get_listings() -> i32 {
			use $crate::DynamicListings;
			let result = __source().get_dynamic_listings();
			__handle_result(result)
		}
	};

	(@single DynamicFilters) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_filters")]
		pub unsafe extern "C" fn __wasm_get_filters() -> i32 {
			use $crate::DynamicFilters;
			let result = __source().get_dynamic_filters();
			__handle_result(result)
		}
	};

	(@single DynamicSettings) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_settings")]
		pub unsafe extern "C" fn __wasm_get_settings() -> i32 {
			use $crate::DynamicSettings;
			let result = __source().get_dynamic_settings();
			__handle_result(result)
		}
	};

	(@single CoverImageProcessor) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "process_cover_image")]
		pub unsafe extern "C" fn __wasm_process_cover_image(response_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(response) =
				$crate::imports::std::read::<$crate::ImageResponse>(response_descriptor)
			else {
				return -1;
			};

			use $crate::CoverImageProcessor;
			let mut result = __source().process_cover_image(response);
			if let Ok(image_ref) = result.as_mut() {
				image_ref.externally_managed = true;
			}
			__handle_result(result.map(|r| r.rid))
		}
	};

	(@single BaseUrlProvider) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_base_url")]
		pub unsafe extern "C" fn __wasm_get_base_url() -> i32 {
			use $crate::BaseUrlProvider;
			let result = __source().get_base_url();
			__handle_result(result)
		}
	};

	(@single NotificationHandler) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "handle_notification")]
		pub unsafe extern "C" fn __wasm_handle_notification(string_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(notification) =
				$crate::imports::std::read::<$crate::alloc::String>(string_descriptor)
			else {
				return -1;
			};
			use $crate::NotificationHandler;
			__source().handle_notification(notification);
			return 0;
		}
	};

	(@single DeepLinkHandler) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "handle_deep_link")]
		pub unsafe extern "C" fn __wasm_handle_deep_link(string_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(url) =
				$crate::imports::std::read::<$crate::alloc::String>(string_descriptor)
			else {
				return -1;
			};
			use $crate::DeepLinkHandler;
			let result = __source().handle_deep_link(url);
			__handle_result(result)
		}
	};

	(@single BasicLoginHandler) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "handle_basic_login")]
		pub unsafe extern "C" fn __wasm_handle_basic_login(
			key_descriptor: i32,
			username_descriptor: i32,
			password_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(key) =
				$crate::imports::std::read::<$crate::alloc::String>(key_descriptor)
			else {
				return -1;
			};
			let ::core::result::Result::Ok(username) =
				$crate::imports::std::read::<$crate::alloc::String>(username_descriptor)
			else {
				return -2;
			};
			let ::core::result::Result::Ok(password) =
				$crate::imports::std::read::<$crate::alloc::String>(password_descriptor)
			else {
				return -3;
			};
			use $crate::BasicLoginHandler;
			let result = __source().handle_basic_login(key, username, password);
			__handle_result(result)
		}
	};

	(@single WebLoginHandler) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "handle_web_login")]
		pub unsafe extern "C" fn __wasm_handle_web_login(
			key_descriptor: i32,
			keys_descriptor: i32,
			values_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(key) =
				$crate::imports::std::read::<$crate::alloc::String>(key_descriptor)
			else {
				return -1;
			};
			let ::core::result::Result::Ok(keys) = $crate::imports::std::read::<
				$crate::alloc::Vec<$crate::alloc::String>,
			>(keys_descriptor) else {
				return -2;
			};
			let ::core::result::Result::Ok(values) = $crate::imports::std::read::<
				$crate::alloc::Vec<$crate::alloc::String>,
			>(values_descriptor) else {
				return -3;
			};
			use $crate::WebLoginHandler;
			let result = __source().handle_web_login(key, keys.into_iter().zip(values).collect());
			__handle_result(result)
		}
	};

	(@single MigrationHandler) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "handle_key_migration")]
		pub unsafe extern "C" fn __wasm_handle_key_migration(
			key_kind: i32,
			anime_key_descriptor: i32,
			episode_key_descriptor: i32,
		) -> i32 {
			let ::core::result::Result::Ok(anime_key) =
				$crate::imports::std::read::<$crate::alloc::String>(anime_key_descriptor)
			else {
				return -1;
			};
			use $crate::MigrationHandler;
			let result = match key_kind {
				// anime
				0 => __source().handle_anime_migration(anime_key),
				// episode
				1 => {
					let ::core::result::Result::Ok(episode_key) =
						$crate::imports::std::read::<$crate::alloc::String>(episode_key_descriptor)
					else {
						return -2;
					};
					__source().handle_episode_migration(anime_key, episode_key)
				}
				_ => return -3,
			};
			__handle_result(result)
		}
	};

	(@single RecommendationsHandler) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "get_recommended_anime")]
		pub unsafe extern "C" fn __wasm_get_recommended_anime(anime_descriptor: i32) -> i32 {
			let ::core::result::Result::Ok(anime) =
				$crate::imports::std::read::<$crate::Anime>(anime_descriptor)
			else {
				return -1;
			};
			use $crate::RecommendationsHandler;
			let result = __source().get_recommended_anime(anime);
			__handle_result(result)
		}
	};

	(@single SegmentUrlInterceptor) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "intercept_segment_url")]
		pub unsafe extern "C" fn __wasm_intercept_segment_url(
			stream_data_descriptor: i32,
			url_descriptor: i32,
		) -> i32 {
			// a descriptor of 0 means no stream data was provided
			let stream_data = if stream_data_descriptor == 0 {
				None
			} else {
				let ::core::result::Result::Ok(stream_data) =
					$crate::imports::std::read::<$crate::StreamData>(stream_data_descriptor)
				else {
					return -1;
				};
				Some(stream_data)
			};
			let ::core::result::Result::Ok(url) =
				$crate::imports::std::read::<$crate::alloc::String>(url_descriptor)
			else {
				return -2;
			};
			use $crate::SegmentUrlInterceptor;
			let result = __source().intercept_segment_url(stream_data.as_ref(), url);
			__handle_result(::core::result::Result::Ok::<
				$crate::alloc::String,
				$crate::imports::error::KomoreiError,
			>(result))
		}
	};

	(@single SegmentDataInterceptor) => {
		#[unsafe(no_mangle)]
		#[unsafe(export_name = "intercept_segment_data")]
		pub unsafe extern "C" fn __wasm_intercept_segment_data(
			stream_data_descriptor: i32,
			url_descriptor: i32,
			data_descriptor: i32,
		) -> i32 {
			// a descriptor of 0 means no stream data was provided
			let stream_data = if stream_data_descriptor == 0 {
				None
			} else {
				let ::core::result::Result::Ok(stream_data) =
					$crate::imports::std::read::<$crate::StreamData>(stream_data_descriptor)
				else {
					return -1;
				};
				Some(stream_data)
			};
			let ::core::result::Result::Ok(url) =
				$crate::imports::std::read::<$crate::alloc::String>(url_descriptor)
			else {
				return -2;
			};
			let ::core::result::Result::Ok(data) =
				$crate::imports::std::read::<$crate::alloc::Vec<u8>>(data_descriptor)
			else {
				return -3;
			};
			use $crate::SegmentDataInterceptor;
			let result = __source().intercept_segment_data(stream_data.as_ref(), url, &data);
			__handle_result(::core::result::Result::Ok::<
				$crate::alloc::Vec<u8>,
				$crate::imports::error::KomoreiError,
			>(result))
		}
	};
}
