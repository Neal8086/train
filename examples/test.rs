
extern crate train;

use train::os::addr::*;
use train::os::fd::*;
use train::os::socket::*;

fn main() {

    let s = ns_socket(NsAddrFamily::NsInet, NsSocketType::Stream, 0).unwrap();

    println!("flag: {:?}", ns_flags(s).unwrap());

    ns_nonblocking(s);
    ns_reuse(s);
    ns_keepalive(s);
    ns_tcp_nopush(s);

    println!("non-blocking: {:?}", ns_flags(s).unwrap());
    println!("fd: {:?}", s);

    ns_close(s);
}
