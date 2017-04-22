
use libc::{self, c_int};
use super::addr::NsAddressFamily;
use NsError;
use NsResult;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum NsSocketType {
    Stream = libc::SOCK_STREAM,
    Datagram = libc::SOCK_DGRAM,
    SeqPacket = libc::SOCK_SEQPACKET,
    Raw = libc::SOCK_RAW,
    Rdm = libc::SOCK_RDM,
}

pub fn ns_socket(domain: NsAddressFamily, ty: NsSocketType, protocol: c_int) -> NsResult<c_int> {
    let mut flags = ty as c_int;

    if cfg!(target_os = "linux") {
        flags = ty as c_int | libc::O_CLOEXEC;
    }
    
    let fd = unsafe { libc::socket(domain as c_int, flags, protocol) };
    if fd < 0 {
        println!("DEBUG: Create Socket Error: {:?}", fd);
        return Err(NsError::Unknow);
    }

    Ok(fd)
}

pub fn ns_close(fd: c_int) {
    let ret = unsafe { libc::close(fd) };
    println!("DEBUG: Close socket FD: {:?}", ret);
}