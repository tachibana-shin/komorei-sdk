//! Error handling for Komorei source library functions.
use super::{html::HtmlError, js::JsError, net::RequestError};
#[cfg(feature = "json")]
use crate::alloc::rc::Rc;
use crate::{
	alloc::{String, string::ToString},
	imports::canvas::CanvasError,
};
use core::{fmt::Display, str::Utf8Error};

pub type Result<T> = core::result::Result<T, KomoreiError>;

/// An error passed back to the source runner.
#[derive(Debug, Clone)]
pub enum KomoreiError {
	/// This feature is unimplemented.
	Unimplemented,
	/// Pass a message back to the app.
	Message(String),
	/// There was an error making a request.
	RequestError(RequestError),
	/// There was an error performing an HTML operation.
	HtmlError(HtmlError),
	/// There was an error performing a JavaScript operation.
	JsError(JsError),
	/// There was an error handling a canvas operation.
	CanvasError(CanvasError),
	/// There was an error handling UTF-8 data.
	Utf8Error(Utf8Error),
	#[cfg(feature = "json")]
	/// JSON parsing error.
	JsonParseError(Rc<serde_json::Error>),
	/// Deserialization error.
	DeserializeError,
}

impl KomoreiError {
	pub const fn error_code(&self) -> i32 {
		match self {
			Self::Unimplemented => -2,
			Self::RequestError(_) => -3,
			Self::HtmlError(_) => -4,
			Self::JsError(_) => -5,
			Self::CanvasError(_) => -6,
			Self::Utf8Error(_) => -7,
			#[cfg(feature = "json")]
			Self::JsonParseError(_) => -8,
			Self::DeserializeError => -9,
			Self::Message(_) => -1,
		}
	}
}

impl KomoreiError {
	/// Creates a new message error.
	pub fn message<S: Display>(message: S) -> Self {
		Self::Message(message.to_string())
	}
}

impl From<RequestError> for KomoreiError {
	fn from(value: RequestError) -> Self {
		Self::RequestError(value)
	}
}

impl From<HtmlError> for KomoreiError {
	fn from(error: HtmlError) -> KomoreiError {
		KomoreiError::HtmlError(error)
	}
}

impl From<JsError> for KomoreiError {
	fn from(error: JsError) -> KomoreiError {
		KomoreiError::JsError(error)
	}
}

impl From<CanvasError> for KomoreiError {
	fn from(error: CanvasError) -> KomoreiError {
		KomoreiError::CanvasError(error)
	}
}

impl From<Utf8Error> for KomoreiError {
	fn from(error: Utf8Error) -> KomoreiError {
		KomoreiError::Utf8Error(error)
	}
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for KomoreiError {
	fn from(error: serde_json::Error) -> KomoreiError {
		KomoreiError::JsonParseError(Rc::new(error))
	}
}