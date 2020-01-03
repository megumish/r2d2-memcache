extern crate memcache;
extern crate r2d2;

use error::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct MemcacheConnectionManager {
    urls: Vec<String>,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
}

impl MemcacheConnectionManager {
    /// Creates a new `MemcacheConnectionManager`.
    ///
    /// See `memcache::Connection::connect` for a description of the parameter
    /// types.
    pub fn new<C: memcache::Connectable>(target: C) -> MemcacheConnectionManager {
        MemcacheConnectionManager {
            urls: target.get_urls(),
            read_timeout: None,
            write_timeout: None,
        }
    }

    /// Sets the Read Timeout for the memcache client
    ///
    /// See `memcache::Connection::set_read_timeout` for more information
    pub fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.read_timeout = timeout
    }

    /// Sets the Write Timeout for the memcache client
    ///
    /// See `memcache::Connection::set_write_timeout` for more information
    pub fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.write_timeout = timeout
    }

}

impl r2d2::ManageConnection for MemcacheConnectionManager {
    type Connection = memcache::Client;
    type Error = Error;

    fn connect(&self) -> Result<memcache::Client, Error> {
        let conn = memcache::Client::connect(self.urls.clone()).map_err(Error::Other);
        if conn.is_err() {
            return conn;
        }
        let mut client = conn.unwrap();

        // as of memcache 0.14, there is no way for these operations to return an error
        if self.write_timeout.is_some() {
            let _ = client.set_write_timeout(self.write_timeout);
        }
        if self.read_timeout.is_some() {
            let _ = client.set_read_timeout(self.read_timeout);
        }

        return Ok(client)

    }

    fn is_valid(&self, connection: &mut memcache::Client) -> Result<(), Error> {
        match connection.version() {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Other(err)),
        }
    }

    fn has_broken(&self, _connection: &mut memcache::Client) -> bool {
        false
    }
}
