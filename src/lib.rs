pub extern crate memcache;
pub extern crate r2d2;
pub mod connection_manager;
pub mod error;

pub use connection_manager::MemcacheConnectionManager;
