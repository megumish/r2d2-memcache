extern crate memcache;
extern crate r2d2;

use connection_info::{ConnectionInfo, IntoConnectionInfo};
use error::Error;

#[derive(Debug)]
pub struct MemcacheConnectionManager {
    connection_info: ConnectionInfo
}

impl MemcacheConnectionManager {
    /// Creates a new `MemcacheConnectionManager`.
    ///
    /// See `memcache::Connection::connect` for a description of the parameter
    /// types.
    pub fn new<T: IntoConnectionInfo>(params: T)
            -> Result<MemcacheConnectionManager, memcache::MemcacheError>  {
        Ok(MemcacheConnectionManager {
            connection_info: try!(params.into_connection_info())
        })
    }
}

impl r2d2::ManageConnection for MemcacheConnectionManager {
    type Connection = memcache::Client;
    type Error = Error;

    fn connect(&self) -> Result<memcache::Client, Error> {
        memcache::Client::new(self.connection_info.clone().addr.as_str()).map_err(Error::Other)
    }

    fn is_valid(&self, connection: &mut memcache::Client) -> Result<(), Error> {
        match connection.version() {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Other(err))
        }
    }

    fn has_broken(&self, _connection: &mut memcache::Client) -> bool {
        false
    }
}
