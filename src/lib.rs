pub extern crate r2d2;
pub extern crate memcache;
pub mod error;
pub mod connection_manager;

pub use connection_manager::MemcacheConnectionManager;
