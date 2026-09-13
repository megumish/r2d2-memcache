use std::sync::{Arc, OnceLock};

use memcache::{Client, ConnectionManager, MemcacheError, Url};
use scheduled_thread_pool::ScheduledThreadPool;

use crate::error::Error;

/// `memcache::Client` builds an r2d2 pool per server, and every r2d2 pool
/// spawns its own worker threads by default. Clients created here share one.
fn shared_thread_pool() -> Arc<ScheduledThreadPool> {
    static POOL: OnceLock<Arc<ScheduledThreadPool>> = OnceLock::new();
    POOL.get_or_init(|| {
        Arc::new(
            ScheduledThreadPool::builder()
                .num_threads(1)
                .thread_name_pattern("r2d2-memcache-worker-{}")
                .build(),
        )
    })
    .clone()
}

#[derive(Debug)]
pub struct MemcacheConnectionManager {
    urls: Vec<String>,
}

impl MemcacheConnectionManager {
    /// Creates a new `MemcacheConnectionManager`.
    ///
    /// See `memcache::Client::connect` for a description of the parameter
    /// types.
    pub fn new<C: memcache::Connectable>(target: C) -> MemcacheConnectionManager {
        MemcacheConnectionManager {
            urls: target.get_urls(),
        }
    }

    fn build_client(&self) -> Result<Client, MemcacheError> {
        let mut pools = Vec::with_capacity(self.urls.len());
        for url in &self.urls {
            let url = Url::parse(url).map_err(|e| MemcacheError::BadURL(e.to_string()))?;
            // Keep test_on_check_out on: the inner manager has no has_broken
            // detection, so this is what replaces a dead connection.
            let pool = r2d2::Pool::builder()
                .max_size(1)
                .thread_pool(shared_thread_pool())
                .build(ConnectionManager::new(url))
                .map_err(MemcacheError::PoolError)?;
            pools.push(pool);
        }
        Client::with_pools(pools)
    }
}

impl r2d2::ManageConnection for MemcacheConnectionManager {
    type Connection = Client;
    type Error = Error;

    fn connect(&self) -> Result<Client, Error> {
        self.build_client().map_err(Error::Other)
    }

    fn is_valid(&self, connection: &mut Client) -> Result<(), Error> {
        connection.version().map(|_| ()).map_err(Error::Other)
    }

    fn has_broken(&self, _connection: &mut Client) -> bool {
        false
    }
}
