
use std::{net, fmt, mem};
use super::*;

#[derive(Clone, Copy)]
pub struct NsIPv4(pub ns_in4_addr);
#[derive(Clone, Copy)]
pub struct NsIPv6(pub ns_in6_addr);

pub enum NsIP {
    V4(NsIPv4),
    V6(NsIPv6),
}

impl NsIP {
    pub fn new_v4(a: u8, b: u8, c: u8, d: u8) -> NsIP {
        NsIP::V4(NsIPv4::new(a, b, c, d))
    }

    pub fn new_v6(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> NsIP {
        NsIP::V6(NsIPv6::new(a, b, c, d, e, f, g, h))
    }
}

impl NsIPv4 {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> NsIPv4 {
        let ip = (((a as u32) << 24) |
                  ((b as u32) << 16) |
                  ((c as u32) <<  8) |
                   (d as u32)).to_be();

        NsIPv4(ns_in4_addr { s_addr: ip })
    }

    pub fn from_std(std_addr: &net::Ipv4Addr) -> NsIPv4 {
        let bits = std_addr.octets();

        NsIPv4::new(bits[0], bits[1], bits[2], bits[3])
    }

    pub fn any() -> NsIPv4 {
        NsIPv4(ns_in4_addr { s_addr: 0 })
    }

    pub fn octets(&self) -> [u8; 4] {
        let bits = u32::from_be(self.0.s_addr);

        [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8]
    }

    pub fn to_std(&self) -> net::Ipv4Addr {
        let bits = self.octets();

        net::Ipv4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }
}

impl NsIPv6 {
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> NsIPv6 {
        let mut addr: ns_in6_addr = unsafe { mem::zeroed() };
        addr.s6_addr = [(a >> 8) as u8, a as u8,
                        (b >> 8) as u8, b as u8,
                        (c >> 8) as u8, c as u8,
                        (d >> 8) as u8, d as u8,
                        (e >> 8) as u8, e as u8,
                        (f >> 8) as u8, f as u8,
                        (g >> 8) as u8, g as u8,
                        (h >> 8) as u8, h as u8];

        NsIPv6(addr)
    }

    pub fn from_std(std_addr: &net::Ipv6Addr) -> NsIPv6 {
        let s = std_addr.segments();

        NsIPv6::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }

    pub fn segments(&self) -> [u16; 8] {
        let arr = &self.0.s6_addr;

        [
            (arr[0] as u16) << 8 | (arr[1] as u16),
            (arr[2] as u16) << 8 | (arr[3] as u16),
            (arr[4] as u16) << 8 | (arr[5] as u16),
            (arr[6] as u16) << 8 | (arr[7] as u16),
            (arr[8] as u16) << 8 | (arr[9] as u16),
            (arr[10] as u16) << 8 | (arr[11] as u16),
            (arr[12] as u16) << 8 | (arr[13] as u16),
            (arr[14] as u16) << 8 | (arr[15] as u16),
        ]
    }

    pub fn to_std(&self) -> net::Ipv6Addr {
        let s = self.segments();

        net::Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }
}

impl fmt::Display for NsIP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NsIP::V4(ref v) => v.fmt(f),
            NsIP::V6(ref v) => v.fmt(f),
        }
    }
}

impl fmt::Display for NsIPv4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let octets = self.octets();
        write!(fmt, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

impl fmt::Display for NsIPv6 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.to_std().fmt(fmt)
    }
}

impl fmt::Debug for NsIP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
