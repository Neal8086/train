use libc;
use std::{net, mem, ptr};
use std::io::{Error, Result};

fn bind(ip: &[u8], port: u16) -> Result<i32> {

    let ip = (((ip[0] as u32) << 24) |
              ((ip[1] as u32) << 16) |
              ((ip[2] as u32) <<  8) |
              (ip[3] as u32)).to_be();

    let ip_addr = libc::in_addr { s_addr: ip };

    let sock_addr_in = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port.to_be(),
        sin_addr: ip_addr,
        .. unsafe { mem::zeroed() }
    };

    let sock_addr = unsafe { mem::transmute::<libc::sockaddr_in, libc::sockaddr>(sock_addr_in) };
    let sock_addr_len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

    let sock_fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    if sock_fd == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Create socket error: {:?}", err);
        return Err(err);
    }
    
    reuse(sock_fd);
    nonblocking(sock_fd);

    let ret = unsafe { libc::bind(sock_fd, &sock_addr as *const libc::sockaddr, sock_addr_len) };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Bind socket failed, Error: {:?}", err);
        return Err(err);
    }

    return Ok(sock_fd);
}

pub fn listen(ip: &[u8], port: u16) -> Result<i32> {
    let fd = bind(ip, port).unwrap();

    let ret = unsafe { libc::listen(fd, super::LISTEN_BACKLOG as i32) };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Socket listen failed. Error: {:?}", err);
        return Err(err);
    }

    return Ok(fd);
}

pub fn reuse(fd: i32) {
    let ret = unsafe {
            let yes = 1;
            libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_REUSEADDR,
                &yes as *const _ as *const libc::c_void,
                mem::size_of::<libc::c_int>() as libc::socklen_t)
    };

    if ret == -1 {
        println!("DEBUG: Set Re-Use failed, Error: {:?}", Error::last_os_error());
        return;
    }

    println!("DEBUG: Set socket re-use.");
}

pub fn nonblocking(fd: i32) {
    let ret = unsafe { 
        let mut nonblocking: libc::c_ulong = 1;
        libc::ioctl(fd, libc::FIONBIO, &mut nonblocking) 
    };

    if ret == -1 {
        println!("DEBUG: Set Non-blocking failed, Error: {:?}", Error::last_os_error());
        return;
    }

    println!("DEBUG: Set socket non-blocking.");
}

pub fn to_std_socket_addr(addr: &libc::sockaddr) -> net::SocketAddr {
    let addr = unsafe { mem::transmute::<&libc::sockaddr, &libc::sockaddr_in>(addr) };

    let bits = u32::from_be(addr.sin_addr.s_addr);
    let bits_arr = [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8];
    let std_ip = net::Ipv4Addr::new(bits_arr[0], bits_arr[1], bits_arr[2], bits_arr[3]);

    net::SocketAddr::V4(net::SocketAddrV4::new(std_ip, addr.sin_port))
}

pub fn close(fd: i32) {
    unsafe { libc::close(fd) };
    println!("DEBUG: Close socket {:?}", fd);
}