
use std::{mem, net, fmt, hash};
use super::*;


#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum NsAddrFamily {
    Unix = NS_AF_UNIX,
    Inet4 = NS_AF_INET,
    Inet6 = NS_AF_INET6,
}

#[derive(Copy)]
pub struct NsInetAddrV4(pub ns_sockaddr_in4);
#[derive(Copy)]
pub struct NsInetAddrV6(pub ns_sockaddr_in6);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NsInetAddr {
    V4(NsInetAddrV4),
    V6(NsInetAddrV6),
}

#[derive(Copy)]
pub struct NsUnixAddr (pub ns_sockaddr_un, pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NsSocketAddr {
    Inet(NsInetAddr),
}

impl NsInetAddrV4 {
    pub fn new(ip: NsIPv4, port: u16) -> NsInetAddrV4 {
        NsInetAddrV4(ns_sockaddr_in4 {
            sin_family: NsAddrFamily::Inet4 as ns_family_t,
            sin_port: port.to_be(),
            sin_addr: ip.0,
            .. unsafe { mem::zeroed() }
        })
    }

    pub fn from_std(std_addr: &net::SocketAddrV4) -> NsInetAddrV4 {
        NsInetAddrV4(ns_sockaddr_in4 {
            sin_family: NsAddrFamily::Inet4 as ns_family_t,
            sin_port: std_addr.port().to_be(),
            sin_addr: NsIPv4::from_std(std_addr.ip()).0,
            .. unsafe { mem::zeroed() }
        })
    }

    pub fn ip(&self) -> NsIPv4 {
        NsIPv4(self.0.sin_addr)
    }

    pub fn port(&self) -> u16 {
        u16::from_be(self.0.sin_port)
    }

    pub fn to_std(&self) -> net::SocketAddrV4 {
        net::SocketAddrV4::new(self.ip().to_std(), self.port())
    }
}

impl NsInetAddrV6 {
    pub fn new(ip: NsIPv6, port: u16) -> NsInetAddrV6 {
        NsInetAddrV6(ns_sockaddr_in6 {
            sin6_family: NsAddrFamily::Inet6 as ns_family_t,
            sin6_port: port.to_be(),
            sin6_addr: ip.0,
            .. unsafe { mem::zeroed() }
        })
    }

    pub fn from_std(std_addr: &net::SocketAddrV6) -> NsInetAddrV6 {
        NsInetAddrV6(ns_sockaddr_in6 {
            sin6_family: NsAddrFamily::Inet6 as ns_family_t,
            sin6_port: std_addr.port().to_be(),
            sin6_addr: NsIPv6::from_std(std_addr.ip()).0,
            sin6_flowinfo: std_addr.flowinfo(),
            sin6_scope_id: std_addr.scope_id(),
            .. unsafe { mem::zeroed() }
        })
    }

    pub fn ip(&self) -> NsIPv6 {
        NsIPv6(self.0.sin6_addr)
    }

    pub fn port(&self) -> u16 {
        u16::from_be(self.0.sin6_port)
    }

    pub fn to_std(&self) -> net::SocketAddrV6 {
        net::SocketAddrV6::new(
            self.ip().to_std(),
            self.port(),
            self.0.sin6_flowinfo,
            self.0.sin6_scope_id,
        )
    }
}

impl NsInetAddr {
    pub fn new(ip: NsIP, port: u16) -> NsInetAddr {
        match ip {
            NsIP::V4(a) => NsInetAddr::V4(NsInetAddrV4::new(a, port)),
            NsIP::V6(a) => NsInetAddr::V6(NsInetAddrV6::new(a, port)),
        }
    }

    pub fn from_std(std_addr: &net::SocketAddr) -> NsInetAddr {
        match *std_addr {
            net::SocketAddr::V4(ref a) => NsInetAddr::V4(NsInetAddrV4::new(NsIPv4::from_std(a.ip()), a.port())),
            net::SocketAddr::V6(ref a) => NsInetAddr::V6(NsInetAddrV6::new(NsIPv6::from_std(a.ip()), a.port())),
        }
    }

    pub fn to_std(&self) -> net::SocketAddr {
        match *self {
            NsInetAddr::V4(ref a) => net::SocketAddr::V4(a.to_std()),
            NsInetAddr::V6(ref a) => net::SocketAddr::V6(a.to_std()),
        }
    }

    pub fn ip(&self) -> NsIP {
        match *self {
            NsInetAddr::V4(ref a) => NsIP::V4(NsIPv4(a.0.sin_addr)),
            NsInetAddr::V6(ref a) => NsIP::V6(NsIPv6(a.0.sin6_addr)),
        }
    }

    pub fn port(&self) -> u16 {
        match *self {
            NsInetAddr::V4(ref a) => u16::from_be(a.0.sin_port),
            NsInetAddr::V6(ref a) => u16::from_be(a.0.sin6_port),
        }
    }
}

impl fmt::Display for NsInetAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NsInetAddr::V4(ref a) => a.fmt(f),
            NsInetAddr::V6(ref a) => a.fmt(f),
        }
    }
}

impl fmt::Display for NsInetAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip(), self.port())
    }
}

impl fmt::Debug for NsInetAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for NsInetAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip(), self.port())
    }
}

impl fmt::Debug for NsInetAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Clone for NsInetAddrV4 {
    fn clone(&self) -> NsInetAddrV4 { *self }
}

impl Clone for NsInetAddrV6 {
    fn clone(&self) -> NsInetAddrV6 { *self }
}

impl PartialEq for NsInetAddrV4 {
    fn eq(&self, other: &NsInetAddrV4) -> bool {
        self.0.sin_port == other.0.sin_port &&
        self.0.sin_addr.s_addr == other.0.sin_addr.s_addr
    }
}

impl PartialEq for NsInetAddrV6 {
    fn eq(&self, other: &NsInetAddrV6) -> bool {
        self.0.sin6_port == other.0.sin6_port &&
        self.0.sin6_addr.s6_addr == other.0.sin6_addr.s6_addr &&
        self.0.sin6_flowinfo == other.0.sin6_flowinfo &&
        self.0.sin6_scope_id == other.0.sin6_scope_id
    }
}

impl Eq for NsInetAddrV4 {}
impl Eq for NsInetAddrV6 {}

impl hash::Hash for NsInetAddrV4 {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        (self.0.sin_port, self.0.sin_addr.s_addr).hash(s)
    }
}

impl hash::Hash for NsInetAddrV6 {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        (self.0.sin6_port, &self.0.sin6_addr.s6_addr,
         self.0.sin6_flowinfo, self.0.sin6_scope_id).hash(s)
    }
}

impl Clone for NsUnixAddr {
    fn clone(&self) -> NsUnixAddr { *self }
}

impl NsSocketAddr {
    pub fn new_inet(addr: NsInetAddr) -> NsSocketAddr {
        NsSocketAddr::Inet(addr)
    }

    pub fn family(&self) -> NsAddrFamily {
        match *self {
            NsSocketAddr::Inet(NsInetAddr::V4(..)) => NsAddrFamily::Inet4,
            NsSocketAddr::Inet(NsInetAddr::V6(..)) => NsAddrFamily::Inet6,
        }
    }

    pub unsafe fn as_ffi(&self) -> (&ns_sockaddr, ns_socklen_t) {
        match *self {
            NsSocketAddr::Inet(NsInetAddr::V4(ref a)) => (mem::transmute(a), mem::size_of::<ns_sockaddr_in4>() as ns_socklen_t),
            NsSocketAddr::Inet(NsInetAddr::V6(ref a)) => (mem::transmute(a), mem::size_of::<ns_sockaddr_in6>() as ns_socklen_t),
        }
    }
}

impl fmt::Display for NsSocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NsSocketAddr::Inet(ref a) => a.fmt(f),
        }
    }
}
