extern crate memcache;

use std::error;
use std::error::Error as _StdError;
use std::fmt;

/// A unified enum of errors by memcache::Connection
#[derive(Debug)]
pub enum Error {
    /// A memcache::MemcacheError
    Other(memcache::MemcacheError)
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.cause() {
            Some(cause) => write!(fmt, "{}: {}", self.description(), cause),
            None => write!(fmt, "{}", self.description())
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Other(ref err) => err.cause()
        }
    }
}
