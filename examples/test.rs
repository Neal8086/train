
extern crate train;

use train::os::addr::*;
use train::os::socket::*;

fn main() {

    let s = NsSocket::new(NsAddressFamily::NsInet, NsSocketType::Stream, 0);

    println!("{:?}", s);
}
