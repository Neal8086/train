
use libc::{self, c_int, socket, close};
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
pub struct NsSocket {
    fd: c_int,
}

impl NsSocket {

    pub fn new(domain: NsAddressFamily, ty: NsSocketType, protocol: c_int) -> NsSocket {
        let fd = unsafe { socket(domain as c_int, ty as c_int, protocol) };

        NsSocket { fd: fd }
    }

    pub fn get_flags(fd: c_int) -> Result<c_int, c_int> {
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };

        if flags < 0 {
            print!("DEBUG:Can not get fd flag: {:?}", flags);
            return Err(0);
        }
        
        Ok(flags)
    }

    pub fn set_nonblocking(fd: c_int) -> Result<c_int, c_int> {
        let mut flags = Self::get_flags(fd).unwrap();

        flags |= libc::O_NONBLOCK;
        if unsafe { libc::fcntl(fd, libc::F_SETFL, flags) } == 0 {
            println!("DEBUG: Set FD failed");
            
            return Err(0);
        }

        Ok(0)
    }

    pub fn fd(&self) -> c_int {
        self.fd
    }

}

impl fmt::Display for NsSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fd)
    }
}

impl Drop for NsSocket {
     fn drop(&mut self) {
         let ret = unsafe { close(self.fd) };

         println!("DEBUG::Closed FD Result: {}", ret);
     }
}