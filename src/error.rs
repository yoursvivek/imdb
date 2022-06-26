use std::error;
use std::fmt;
use std::io;

/// Error type for `imdb`.
///
/// Errors occur mainly during io or http requests.
#[derive(Debug)]
pub enum Error {
    /// Error from IO operation
    IOError(io::Error),
    /// Error from reqwest library
    ReqwestError(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IOError(ref err) => write!(f, "IO error: {}", err),
            Error::ReqwestError(ref err) => write!(f, "Reqwest error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::IOError(ref err) => Some(err),
            Error::ReqwestError(ref err) => Some(err),
        }
    }
}
