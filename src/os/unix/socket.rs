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

pub struct NsSocket(NsFd);

impl NsSocket {

    pub fn new(domain: NsAddressFamily, ty: NsSocketType, protocol: c_int) -> NsSocket {
        let fd = unsafe { socket(domain as c_int, ty as c_int, protocol) };

        NsSocket(NsFd::new(fd))
    }

    pub fn fd(&self) -> NsFd {
        self.0
    }

}
