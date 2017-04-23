
use winapi::{self, c_int};

pub const NS_AF_UNIX: c_int = -1;
pub const NS_AF_INET: c_int = winapi::ws2def::AF_INET;
pub const NS_AF_INET6: c_int = winapi::ws2def::AF_INET6;

pub type NsSockAddrV4 = winapi::ws2def::SOCKADDR_IN;
pub type NsSockAddrV6 = winapi::ws2ipdef::sockaddr_in6;