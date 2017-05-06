use std::{mem, io};
use libc;
use NsError;
use NsResult;
use super::*;


// ioctl(FIONBIO) sets a non-blocking mode with the single syscall
// while fcntl(F_SETFL, O_NONBLOCK) needs to learn the current state using fcntl(F_GETFL).

pub fn ns_nonblocking(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe { 
        let mut nonblocking: ns_ulong = 1;
        libc::ioctl(fd, NS_FIONBIO, &mut nonblocking) 
    };
    if ret == -1 {
        println!("DEBUG: Set Non-blocking FD failed");
        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_blocking(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe { 
        let mut blocking: ns_ulong = 0;
        libc::ioctl(fd, NS_FIONBIO, &mut blocking) 
    };
    if ret == -1 {
        println!("DEBUG: Set blocking FD failed");
        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_reuse(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe {
        let yes = 1;
        libc::setsockopt(
            fd,
            NS_SOL_SOCKET,
            NS_SO_REUSEADDR,
            &yes as *const _ as *const ns_void,
            mem::size_of::<ns_int>() as ns_socklen_t)
    };
    if ret < 0 {
        println!("DEBUG: Set socket opt re-use failed!");
        println!("DEBUG: {:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_keepalive(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe {
        let yes = 1;
        libc::setsockopt(
            fd,
            NS_SOL_SOCKET,
            NS_SO_KEEPALIVE,
            &yes as *const _ as *const ns_void,
            mem::size_of::<ns_int>() as ns_socklen_t)
    };
    if ret < 0 {
        println!("DEBUG: Set socket opt keepalive failed!");
        println!("DEBUG: {:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_linger(fd: ns_fd, onoff: ns_int, linger: ns_int) -> NsResult<ns_int> {
    let l = ns_linger {
        l_onoff: onoff,
        l_linger: linger
    };
    let ptr: *const ns_void = unsafe { mem::transmute(&l) };
    let len = mem::size_of::<ns_linger>() as ns_socklen_t;

    let ret = unsafe { libc::setsockopt(fd, NS_SOL_SOCKET, NS_SO_LINGER, ptr, len) };
    if ret < 0 {
        println!("DEBUG: Set socket opt failed!");
        println!("DEBUG: {:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn ns_tcp_nopush(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe {
        let yes = 1;
        libc::setsockopt(
            fd,
            NS_IPPROTO_TCP,
            NS_TCP_CORK,
            &yes as *const _ as *const ns_void,
            mem::size_of::<ns_int>() as ns_socklen_t)
    };
    if ret < 0 {
        println!("DEBUG: Set linux socket opt TCP nopush failed!");
        println!("DEBUG: {:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn ns_tcp_push(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe {
        let yes = 0;
        libc::setsockopt(
            fd,
            NS_IPPROTO_TCP,
            NS_TCP_CORK,
            &yes as *const _ as *const ns_void,
            mem::size_of::<ns_int>() as ns_socklen_t)
    };
    if ret < 0 {
        println!("DEBUG: Set linux socket opt TCP push failed!");
        println!("DEBUG: {:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

#[cfg(any(target_os = "macos", target_os = "ios", 
          target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub fn ns_tcp_nopush(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe {
        let yes = 1;
        libc::setsockopt(
            fd,
            NS_IPPROTO_TCP,
            NS_TCP_NOPUSH,
            &yes as *const _ as *const ns_void,
            mem::size_of::<ns_int>() as ns_socklen_t)
    };
    if ret < 0 {
        println!("DEBUG: Set MacOS socket opt TCP nopush failed!");
        println!("DEBUG: {:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

#[cfg(any(target_os = "macos", target_os = "ios", 
          target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub fn ns_tcp_push(fd: ns_fd) -> NsResult<ns_int> {
    let ret = unsafe {
        let yes = 0;
        libc::setsockopt(
            fd,
            NS_IPPROTO_TCP,
            NS_TCP_NOPUSH,
            &yes as *const _ as *const ns_void,
            mem::size_of::<ns_int>() as ns_socklen_t)
    };
    if ret < 0 {
        println!("DEBUG: Set MacOS socket opt TCP push failed!");
        println!("DEBUG: {:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}