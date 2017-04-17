
use libc::{self, c_int, socket};
use super::fd::NsFd;
use super::addr::NsAddressFamily;
use std::{fmt};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum NsSocketType {
    Stream = libc::SOCK_STREAM,
    Datagram = libc::SOCK_DGRAM,
    SeqPacket = libc::SOCK_SEQPACKET,
    Raw = libc::SOCK_RAW,
    Rdm = libc::SOCK_RDM,
}

#[derive(Debug)]
pub struct NsSocket(NsFd);

impl NsSocket {

    pub fn new(domain: NsAddressFamily, ty: NsSocketType, protocol: c_int) -> NsSocket {
        let fd = unsafe { socket(domain as c_int, ty as c_int, protocol) };

        NsSocket(NsFd::new(fd))
    }


}

impl fmt::Display for NsSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
