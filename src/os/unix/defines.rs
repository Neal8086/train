use libc::{self, c_int, c_ulong};

pub const NS_AF_UNIX: c_int = libc::AF_UNIX;
pub const NS_AF_INET: c_int = libc::AF_INET;
pub const NS_AF_INET6: c_int = libc::AF_INET6;

pub const NS_SOCK_STREAM: c_int = libc::SOCK_STREAM;
pub const NS_SOCK_DGRAM: c_int = libc::SOCK_DGRAM;
pub const NS_SOCK_RAW: c_int = libc::SOCK_RAW;
pub const NS_SOCK_RDM: c_int = libc::SOCK_RDM;
pub const NS_SOCK_SEQPACKET: c_int = libc::SOCK_SEQPACKET;

pub const NS_O_CLOEXEC: c_int = libc::O_CLOEXEC;
pub const NS_O_NONBLOCK: c_int = libc::O_NONBLOCK;

pub const NS_FIONBIO: c_ulong = libc::FIONBIO;

pub const NS_SOL_SOCKET: c_int = libc::SOL_SOCKET;
pub const NS_SO_REUSEADDR: c_int = libc::SO_REUSEADDR;
pub const NS_SO_KEEPALIVE: c_int = libc::SO_KEEPALIVE;
pub const NS_SO_LINGER: c_int = libc::SO_LINGER;

pub const NS_IPPROTO_TCP: c_int = libc::IPPROTO_TCP;
pub const NS_IPPROTO_IP: c_int = libc::IPPROTO_IP;
pub const NS_IPPROTO_ICMP: c_int = libc::IPPROTO_ICMP;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub const NS_TCP_CORK: c_int = libc::TCP_CORK;
#[cfg(any(target_os = "macos", target_os = "ios", 
          target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub const NS_TCP_NOPUSH: c_int = 0x04;

pub const NS_TCP_NODELAY: c_int = libc::TCP_NODELAY;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub const NS_TCP_MAXSEG: c_int = libc::TCP_MAXSEG;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub const NS_TCP_MAXSEG: c_int = 2;

pub type ns_int = c_int;
pub type ns_ulong = c_ulong;
pub type ns_fd = c_int;
pub type ns_family_t = libc::sa_family_t;
pub type ns_socklen_t = libc::socklen_t;
pub type ns_void = libc::c_void;

pub type ns_in4_addr = libc::in_addr;
pub type ns_in6_addr = libc::in6_addr;
pub type ns_sockaddr = libc::sockaddr;
pub type ns_sockaddr_in4 = libc::sockaddr_in;
pub type ns_sockaddr_in6 = libc::sockaddr_in6;
pub type ns_sockaddr_un = libc::sockaddr_un;

pub type ns_linger = libc::linger;
