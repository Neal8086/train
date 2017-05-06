extern crate train;

use train::os::addr::*;
use train::os::fd::*;
use train::os::socket::*;
use train::os::socketopt::*;

fn main() {
    test_socket();
    
    test_socketpair();
}

fn test_socket() {
    println!("Socket Test --------------------------");

    let s = ns_socket(NsAddrFamily::Inet4, NsSockType::Stream, 0).unwrap();

    println!("Create Socket FD: {:?}", s);
    println!("flag: {:?}", ns_flags(s).unwrap());

    println!("Set Non-Blocking: {:?}", ns_nonblocking(s).unwrap());
    
    println!("Set Reuse: {:?}", ns_reuse(s).unwrap());
    println!("Set Keepalive: {:?}", ns_keepalive(s).unwrap());
    println!("Set TCP no push: {:?}", ns_tcp_nopush(s).unwrap());
    println!("Scoket FD flags: {:?}", ns_flags(s).unwrap());
    
    println!("Close: {:?}", ns_close_socket(s).unwrap());
}

fn test_socketpair() {
    println!("Socketpair Test ---------------------------");

    let fds = ns_socketpair(NsAddrFamily::Unix, NsSockType::Stream, 0).unwrap();
    println!("Create socketpair FDs: {:?}", fds);

    println!("Set Non-Blocking. FD: {:?}, Result: {:?}", fds.0, ns_nonblocking(fds.0).unwrap());
    println!("Set Non-Blocking. FD: {:?}, Result: {:?}", fds.1, ns_nonblocking(fds.1).unwrap());

    println!("Scoket FD flags. FD: {:?}, Result: {:?}", fds.0, ns_flags(fds.0).unwrap());
    println!("Scoket FD flags. FD: {:?}, Result: {:?}", fds.1, ns_flags(fds.1).unwrap());

    println!("Close. FD: {:?}, Result: {:?}", fds.0, ns_close_socket(fds.0).unwrap());
    //println!("Close. FD: {:?}, Result: {:?}", fds.1, ns_close_socket(fds.0).unwrap());
}