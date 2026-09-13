pub use memcache;
pub use r2d2;

pub mod connection_manager;
pub mod error;

pub use connection_manager::MemcacheConnectionManager;
