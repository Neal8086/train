use std::{mem};
use libc;
use NsError;
use NsResult;
use super::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum NsSockType {
    Stream = NS_SOCK_STREAM,
    Datagram = NS_SOCK_DGRAM,
    SeqPacket = NS_SOCK_SEQPACKET,
    Raw = NS_SOCK_RAW,
    Rdm = NS_SOCK_RDM,
}

pub fn ns_socket(domain: NsAddrFamily, ty: NsSockType, protocol: ns_int) -> NsResult<ns_fd> {
    let mut flags = ty as ns_int;

    if cfg!(target_os = "linux") {
        flags = ty as ns_int | NS_O_CLOEXEC;
    }
    
    let fd = unsafe { libc::socket(domain as ns_int, flags, protocol) };
    if fd < 0 {
        println!("DEBUG: Create Socket Error: {:?}", fd);
        return Err(NsError::Unknow);
    }

    Ok(fd)
}

pub fn ns_socketpair(domain: NsAddrFamily, ty: NsSockType, protocol: ns_int) ->NsResult<(ns_fd, ns_fd)> {
    let mut fds = [-1, -1];
    let ret = unsafe { libc::socketpair(domain as ns_int, ty as ns_int, protocol, fds.as_mut_ptr()) };
    if ret == -1 {
        println!("DEBUG: Create socketpair failed! {:?}", ret);
        return Err(NsError::Unknow);
    }

    Ok((fds[0], fds[1]))
}

pub fn ns_bind(fd: ns_fd, addr: &NsSocketAddr) -> NsResult<ns_int> {
    let ret = unsafe {
        let (addr_ptr, len) = addr.as_ffi();
        libc::bind(fd, addr_ptr, len)
    };

    if ret == -1 {
        println!("DEBUG: Bind socket to {:?} error.", addr);
        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_listen(fd: ns_fd, backlog: ns_int) -> NsResult<ns_int> {
    let ret = unsafe { libc::listen(fd, backlog) };
    if ret == -1 {
        println!("DEBUG: Listen socket failed: {:?}", ret);
        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_accept(fd: ns_fd) -> NsResult<(ns_fd, NsSocketAddr, ns_socklen_t)> {
    let mut len: ns_socklen_t = 0;
    let mut addr: ns_sockaddr = unsafe { mem::zeroed() };

    let client_fd = unsafe { libc::accept(fd, &mut addr, &mut len) };
    if client_fd == -1 {
        println!("DEBUG: Accept socket failed. {:?}", client_fd);
        return Err(NsError::Unknow);
    }

    let addr_in = unsafe { mem::transmute::<ns_sockaddr, ns_sockaddr_in4>(addr) };

    Ok((client_fd, NsSocketAddr::Inet(NsInetAddr::V4(NsInetAddrV4(addr_in))), len))
}

pub fn ns_shutdown_socket(fd: ns_fd, how: ns_int) -> NsResult<ns_int> {
    let ret = unsafe { libc::shutdown(fd, how) };
    
    println!("DEBUG: Shutdown socket, {:?}", ret);
    
    if ret == -1 {
        println!("DEBUG: Shutdown socket failed. {:?}", ret);
        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_close_socket(fd: ns_fd) -> NsResult<ns_int> {
    println!("DEBUG: Close socket FD, {}", fd);

    let ret = unsafe { libc::close(fd) };
    if ret == -1 {
        println!("DEBUG: Close socket failed. {:?}", ret);
        return Err(NsError::Unknow);
    }

    Ok(ret)
}