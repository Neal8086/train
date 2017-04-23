
use super::{NsSockAddrV4, NS_AF_UNIX, NS_AF_INET, NS_AF_INET6};

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum NsAddressFamily {
    NsUnix = NS_AF_UNIX,
    NsInet = NS_AF_INET,
    NsInet6 = NS_AF_INET6,
}

pub struct NsInetAddrV4(pub NsSockAddrV4);

impl NsInetAddrV4 {
    pub fn new() {
        println!("NsInetAddrV4 new...");
    }
}
