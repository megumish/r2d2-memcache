use memcache::MemcacheError;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::vec::IntoIter;
use std::io;

type MemcacheValue<T> = Result<T, MemcacheError>;

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub addr: String
}

pub trait IntoConnectionInfo {
    fn into_connection_info(self) -> MemcacheValue<ConnectionInfo>;
}

impl IntoConnectionInfo for ConnectionInfo {
    fn into_connection_info(self) -> MemcacheValue<ConnectionInfo> {
        Ok(self)
    }
}

impl<'a> IntoConnectionInfo for &'a str {
    fn into_connection_info(self) -> MemcacheValue<ConnectionInfo> {
        Ok(ConnectionInfo { addr: String::from(self) })
    }
}

impl ToSocketAddrs for ConnectionInfo {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
        self.addr.as_str().to_socket_addrs()
    }
}
