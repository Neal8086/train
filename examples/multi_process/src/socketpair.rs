use libc;
use libc::c_ulong;
use std::io::{Error, Result};


// mac os
pub const TIOCEXCL: c_ulong = 0x2000740d;
pub const TIOCNXCL: c_ulong = 0x2000740e;
pub const TIOCSCTTY: c_ulong = 0x20007461;
pub const TIOCGPGRP: c_ulong = 0x40047477;
pub const TIOCSPGRP: c_ulong = 0x80047476;
pub const TIOCOUTQ: c_ulong = 0x40047473;
pub const TIOCSTI: c_ulong = 0x80017472;
pub const TIOCGWINSZ: c_ulong = 0x40087468;
pub const TIOCSWINSZ: c_ulong = 0x80087467;
pub const TIOCMGET: c_ulong = 0x4004746a;
pub const TIOCMBIS: c_ulong = 0x8004746c;
pub const TIOCMBIC: c_ulong = 0x8004746b;
pub const TIOCMSET: c_ulong = 0x8004746d;
pub const FIONREAD: c_ulong = 0x4004667f;
pub const TIOCCONS: c_ulong = 0x80047462;
pub const TIOCPKT: c_ulong = 0x80047470;
pub const FIONBIO: c_ulong = 0x8004667e;
pub const TIOCNOTTY: c_ulong = 0x20007471;
pub const TIOCSETD: c_ulong = 0x8004741b;
pub const TIOCGETD: c_ulong = 0x4004741a;
pub const FIONCLEX: c_ulong = 0x20006602;
pub const FIOCLEX: c_ulong = 0x20006601;
pub const FIOASYNC: c_ulong = 0x8004667d;


pub fn socketpair() -> Result<(libc::c_int, libc::c_int)> {
    let mut fds = [-1, -1];
    let ret = unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Create socketpair failed! {:?}", err);
        return Err(err);
    }

    Ok((fds[0], fds[1]))
}

pub fn nonblocking(fd: i32) {
    let ret = unsafe { 
        let mut nonblocking: libc::c_ulong = 1;
        libc::ioctl(fd, libc::FIONBIO, &mut nonblocking) 
    };

    if ret == -1 {
        println!("DEBUG: Set Non-blocking failed, Error: {:?}", Error::last_os_error());
        return;
    }

    println!("DEBUG: Set socket non-blocking.");
}

pub fn fio_async(fd: i32, on: bool) {
    let status = if on { 1 } else { 0 };
    let ret = unsafe { libc::ioctl(fd, FIOASYNC, &status) };

    if ret == -1 {
        println!("ERROR: Set FIOASYNC failed, Error: {:?}", Error::last_os_error());
        return;
    }

    println!("DEBUG: Set socket FIOASYNC.");
}

pub fn fcntl_set(fd: i32, key: i32, v: i32) {
    let ret = unsafe { libc::fcntl(fd, key, v) };

    if ret == -1 {
        println!("ERROR: fcntl {:?} set {:?} failed, Error: {:?}", fd, key, Error::last_os_error());
        return;
    }

    println!("DEBUG: fcntl set.");
}

pub fn write(fd: i32, msg: &str) {
    let buf :Vec<u8> = msg.as_bytes().to_vec();
    let ret = unsafe { libc::write(fd, &buf as *const _ as *const libc::c_void, buf.len()) };

    if ret == -1 {
        println!("ERROR: write to socket failed, Msg: {:?} Error: {:?}", msg, Error::last_os_error());
        return;
    }

    println!("DEBUG: write to socket: {:?}.", msg);
}

pub fn send(fd: i32, msg: &str) {
    let buf :Vec<u8> = msg.as_bytes().to_vec();
    let ret = unsafe { libc::send(fd, &buf as *const _ as *const libc::c_void, buf.len(), 0) };

    if ret == -1 {
        println!("ERROR: send to socket failed, Msg: {:?} Error: {:?}", msg, Error::last_os_error());
        return;
    }

    println!("DEBUG: send to socket: {:?}.", msg);
}