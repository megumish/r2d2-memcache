extern crate memcache;
extern crate r2d2;

use error::Error;

#[derive(Debug)]
pub struct MemcacheConnectionManager<'a> {
    urls: Vec<&'a str>
}

impl<'a> MemcacheConnectionManager<'a> {
    /// Creates a new `MemcacheConnectionManager`.
    ///
    /// See `memcache::Connection::connect` for a description of the parameter
    /// types.
    pub fn new<C: memcache::Connectable<'a>>(target: C) -> Result<MemcacheConnectionManager<'a>, memcache::MemcacheError>  {
        Ok(MemcacheConnectionManager {
            urls: target.get_urls()
        })
    }
}

impl r2d2::ManageConnection for MemcacheConnectionManager<'static> {
    type Connection = memcache::Client;
    type Error = Error;

    fn connect(&self) -> Result<memcache::Client, Error> {
        memcache::Client::new(self.urls.clone()).map_err(Error::Other)
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
