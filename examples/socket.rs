extern crate train;

use train::os::ip::*;
use train::os::addr::*;
use train::os::fd::*;
use train::os::socket::*;
use train::os::socketopt::*;

fn main() {
    test_socket();
    
    test_socketpair();

    test_socket_accept();
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
    
    println!("Close socket: {:?}", ns_close_socket(s).unwrap());
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

fn test_socket_accept() {
    println!("Scoket Accept Test ------------------------------");

    let s = ns_socket(NsAddrFamily::Inet4, NsSockType::Stream, 0).unwrap();
    let addr = NsSocketAddr::Inet(NsInetAddr::new(NsIP::new_v4(127, 0, 0, 1), 3000));

    println!("Create Socket FD: {:?}", s);
    println!("flag: {:?}", ns_flags(s).unwrap());

    // println!("Set Non-Blocking: {:?}", ns_nonblocking(s).unwrap());
    
    println!("Set Reuse: {:?}", ns_reuse(s).unwrap());
    println!("Set Keepalive: {:?}", ns_keepalive(s).unwrap());
    println!("Set TCP no push: {:?}", ns_tcp_nopush(s).unwrap());
    println!("Scoket FD flags: {:?}", ns_flags(s).unwrap());

    println!("Binding socket: {:?}", ns_bind(s, &addr).unwrap());

    println!("Listen socket: On: {:?}, Result: {:?}", addr, ns_listen(s, 5).unwrap());

    println!("Waiting accept...");

    let (client_fd, addr, len) = ns_accept(s).unwrap();

    println!(">>> A incoming, Client FD: {:?}, Address: {:?}, Length: {:?}", client_fd, addr, len);

    println!("Close client connection: {:?}", ns_close_socket(client_fd).unwrap());
    println!("Close socket: {:?}", ns_close_socket(s).unwrap());
}