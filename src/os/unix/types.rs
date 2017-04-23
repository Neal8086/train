use libc::{self, c_int};

pub const NS_AF_UNIX: c_int = libc::AF_UNIX;
pub const NS_AF_INET: c_int = libc::AF_INET;
pub const NS_AF_INET6: c_int = libc::AF_INET6;

pub type NsSockAddrV4 = libc::sockaddr_in;
pub type NsSockAddrV6 = libc::sockaddr_in6;