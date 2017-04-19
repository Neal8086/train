
extern crate train;

use train::os::addr::*;
use train::os::socket::*;

fn main() {

    let s = NsSocket::new(NsAddressFamily::NsInet, NsSocketType::Stream, 0);

    println!("flag: {:?}", NsSocket::get_flags(s.fd()).unwrap());

    NsSocket::set_nonblocking(s.fd());

    println!("non-blocking: {:?}", NsSocket::get_flags(s.fd()).unwrap());
    println!("fd: {:?}", s);
    
}
