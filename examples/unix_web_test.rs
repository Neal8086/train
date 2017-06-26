#![allow(dead_code)]
#![allow(unused_imports)]
extern crate libc;

use std::{mem, ptr, fmt, net, thread, time};
use std::io::Error;
use std::fmt::Debug;
use std::io::Write;
use std::io::stdout;

const MAX_EVENTS: usize = 512;
const MAX_BUFFER: usize = 4096;
const LISTEN_BACKLOG: usize = 512;
const PORT: u16 = 3000;


fn main() {
    println!("Kevent web sample");

    test_500_request();
}

fn get_server_addr() -> (libc::sockaddr, libc::socklen_t) {
    let ip: [u8; 4] = [127, 0, 0, 1]; 

    let ip = (((ip[0] as u32) << 24) |
              ((ip[1] as u32) << 16) |
              ((ip[2] as u32) <<  8) |
              (ip[3] as u32)).to_be();

    let ip_addr = libc::in_addr { s_addr: ip };

    let sock_addr_in = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: PORT.to_be(),
        sin_addr: ip_addr,
        .. unsafe { mem::zeroed() }
    };

    let addr = unsafe { mem::transmute::<libc::sockaddr_in, libc::sockaddr>(sock_addr_in) };
    let addr_len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

    (addr, addr_len)
}

fn connect() -> i32 {
    let (addr, addr_len) = get_server_addr();
    let sock_fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    let ret = unsafe { libc::connect(sock_fd, &addr as *const libc::sockaddr, addr_len) };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Connect failed. Error: {:?}", err);
        return -1;
    }

    sock_fd
}

fn receive_data(fd: i32) {
    let mut buf: [u8; MAX_BUFFER] = unsafe { mem::uninitialized() };

    let recv_len = unsafe { libc::recv(fd, buf.as_mut_ptr() as *mut libc::c_void, MAX_BUFFER, 0) };

    let data_buf = &buf[0..recv_len as usize];

    println!("DEBUG: Receive data: {:?}", String::from_utf8_lossy(data_buf));
}

fn send_data(fd: i32, txt: &str) {
    let buf = txt.as_bytes();
    unsafe { libc::send(fd, buf as *const _ as *const libc::c_void, buf.len(), 0) };
}

fn test_500_request() {
    println!("500 times request --------------------------------------");
    let mut threads = vec![];

    let send_str = "500 times request test\n";
    
    for _ in 0..10 {
        threads.push(thread::spawn(move || {
            for _ in 0..50 {
                let fd = connect();

                send_data(fd, send_str);
                receive_data(fd);

                unsafe { libc::close(fd) };
            }
        }));
    }

    for t in threads {
        let _ = t.join();
    }

    println!("DONE: 500 times request --------------------------------------");
}

fn test_big_data_request() {
    println!("Big data request test ---------------------------------------");



    println!("DONE: Big data request test ---------------------------------------");
}