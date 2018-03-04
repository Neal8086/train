#[macro_use]
extern crate log;
extern crate simple_logger;

extern crate train;

use train::{Nest, Config};

fn main() {
    simple_logger::init().unwrap();

    trace!("Lib Test");

    let config = Config {
        addr: "127.0.0.1:8080".parse().unwrap(),
        worker_processes: 2,
        worker_connections: 1024,
    };

    let _ = Nest::new(&config).listen();
}
