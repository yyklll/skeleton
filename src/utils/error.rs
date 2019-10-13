use failure::Fail;

use std::io;

#[derive(Fail, Debug)]
pub enum Error {
  /// IO error
  #[fail(display = "IO error: {}", _0)]
  Io(#[cause] io::Error),
  /// Serialization or deserialization error
  #[fail(display = "serde_json error: {}", _0)]
  Serde(#[cause] serde_json::Error),
  /// Unexpected error type
  #[fail(display = "Unexpected Error Type")]
  UnexpectedErrorType,
  /// Error with a string message
  #[fail(display = "{}", _0)]
  StringError(String),
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::Io(err)
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Error {
    Error::Serde(err)
  }
}

pub type Result<T> = std::result::Result<T, Error>;
