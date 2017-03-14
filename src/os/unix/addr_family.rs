use libc;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum AddressFamily {
    Unix = consts::AF_UNIX,
    Inet = consts::AF_INET,
    Inet6 = consts::AF_INET6,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Netlink = consts::AF_NETLINK,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Packet = consts::AF_PACKET,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    System = consts::AF_SYSTEM,
}