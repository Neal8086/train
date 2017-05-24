extern crate train;

use train::event::traits::*;
use train::event::modules::*;


fn main() {
    println!("Event ...");

    let kq = NsKqueue::new().unwrap();

    println!("New Kqueue: {}", kq.fd);

    println!("Notify init: {}", kq.notify_init().unwrap());

}