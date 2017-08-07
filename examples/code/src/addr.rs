extern crate train;

use train::os::ip::*;
use train::os::addr::*;


fn main() {

    let addr_v4 = NsInetAddrV4::new(NsIPv4::new(127, 0, 0, 1), 3000);
    let addr = NsInetAddr::new(NsIP::new_v4(127, 0, 0, 1), 3000);

    println!("Addr V4: {:?}", addr_v4);
    println!("Addr enum V4: {:?}", addr);
    println!("Addr to std: {:?}", addr.to_std());
}