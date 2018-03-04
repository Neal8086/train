
use std::net::SocketAddr;

#[derive(Copy)]
pub struct Config {
    pub addr: SocketAddr,
    pub worker_processes: u16,
    pub worker_connections: u16,

}

impl Clone for Config {
     fn clone(&self) -> Self { *self }
}