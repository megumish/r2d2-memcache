use std::error;
use std::fmt;

/// A unified enum of errors by memcache::Client
#[derive(Debug)]
pub enum Error {
    /// A memcache::MemcacheError
    Other(memcache::MemcacheError),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Other(ref err) => write!(fmt, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Other(ref err) => Some(err),
        }
    }
}

impl From<memcache::MemcacheError> for Error {
    fn from(err: memcache::MemcacheError) -> Error {
        Error::Other(err)
    }
}
