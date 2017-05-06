extern crate train;

use train::os::ip::*;

fn main() {

    let v4 = NsIPv4::new(127, 0, 0, 1);
    let ip = NsIP::V4(v4);

    let ip_v4 = NsIP::new_v4(127, 0, 0, 1);

    println!("IP V4: {:?}", ip);
    println!("IP enum v4: {:?}", ip_v4);
}