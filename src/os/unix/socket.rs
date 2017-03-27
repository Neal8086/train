use libc::{c_int, socket, SOCK_STREAM, SOCK_DGRAM, SOCK_SEQPACKET, SOCK_RAW, SOCK_RDM};
use super::fd::NsFd;
use super::addr::NsAddressFamily;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum NsSocketType {
    Stream = SOCK_STREAM,
    Datagram = SOCK_DGRAM,
    SeqPacket = SOCK_SEQPACKET,
    Raw = SOCK_RAW,
    Rdm = SOCK_RDM,
}

pub struct NsSocket {
    fd: NsFd,
}

impl NsSocket {
    pub fn new(domain: NsAddressFamily, ty: NsSocketType, protocol: c_int) -> NsSocket {

    }
}
