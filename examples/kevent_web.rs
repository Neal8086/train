#![allow(dead_code)]
#![allow(unused_imports)]
extern crate libc;

use std::{mem, ptr, fmt, net, thread, time};
use std::io::Error;
use std::fmt::Debug;

const MAX_EVENTS: usize = 512;
const MAX_BUFFER: usize = 1024;
const LISTEN_BACKLOG: usize = 512;

struct EventQueue {
    pub queue: i32,
    pub fd: i32,
    pub index: usize,
    pub accept_total: usize,
    pub changes: [libc::kevent; MAX_EVENTS],
    pub events: [libc::kevent; MAX_EVENTS],
}


fn main() {
    println!("Kevent test");

    let sock_fd = socket_listen(socket_bind(&[127, 0, 0, 1], 3000).unwrap()).unwrap();

    let event_fd = event_init().unwrap();

    let mut event_queue = EventQueue {
        queue: event_fd,
        fd: sock_fd,
        index: 0,
        accept_total: 0,
        changes: unsafe { mem::uninitialized() },
        events: unsafe { mem::uninitialized() },
    };

    set_event(&mut event_queue, sock_fd, libc::EVFILT_READ, libc::EV_ADD|libc::EV_ENABLE, 0 as *mut libc::c_void);

    process_events(&mut event_queue);

    close(&event_queue);
}

fn socket_bind(ip: &[u8], port: u16) -> Result<i32, Error> {

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
    
    socket_reuse(sock_fd);
    socket_nonblocking(sock_fd);

    let ret = unsafe { libc::bind(sock_fd, &sock_addr as *const libc::sockaddr, sock_addr_len) };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Bind socket failed, Error: {:?}", err);
        return Err(err);
    }

    return Ok(sock_fd);
}

fn socket_listen(fd: i32) -> Result<i32, Error> {
    let ret = unsafe { libc::listen(fd, LISTEN_BACKLOG as i32) };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Socket listen failed. Error: {:?}", err);
        return Err(err);
    }

    return Ok(fd);
}

fn socket_reuse(fd: i32) {
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

fn socket_nonblocking(fd: i32) {
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

fn to_std_socket_addr(addr: &libc::sockaddr) -> net::SocketAddr {
    let addr = unsafe { mem::transmute::<&libc::sockaddr, &libc::sockaddr_in>(addr) };

    let bits = u32::from_be(addr.sin_addr.s_addr);
    let bits_arr = [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8];
    let std_ip = net::Ipv4Addr::new(bits_arr[0], bits_arr[1], bits_arr[2], bits_arr[3]);

    net::SocketAddr::V4(net::SocketAddrV4::new(std_ip, addr.sin_port))
}

fn close_socket(fd: i32) {
    unsafe { libc::close(fd) };
    println!("DEBUG: Close socket {:?}", fd);
}

/*
    when the file descriptor is closed the kqueue automatically deletes 
    its filters so we do not need to delete explicitly the event 
    before the closing the file descriptor.
*/
fn close(eq: &EventQueue) {

    unsafe { libc::close(eq.queue) };
    println!("DEBUG: Close kqueue.");
   
    close_socket(eq.fd);
}

fn event_init() -> Result<i32, Error> {

    let queue_fd = unsafe { libc::kqueue() };
    if queue_fd == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Create kqueue failed. Error: {:?}", err);
        return Err(err);
    }

    return Ok(queue_fd);
}

fn set_event(eq: &mut EventQueue, fd: i32, filter: i16, flags: u16, udata: *mut libc::c_void) {

    if eq.index >= MAX_EVENTS {
        eq.changes = unsafe { mem::uninitialized() };
        eq.index = 0;
    }

    eq.changes[eq.index] = libc::kevent {
        ident: fd as usize,
        filter: filter,
        flags: flags,
        fflags: 0,
        data: 0,
        udata: udata,
    };
    eq.index += 1;

    let ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let ret = unsafe { 
        libc::kevent(eq.queue, eq.changes.as_ptr(), eq.index as i32, ptr::null_mut(), 0, &ts as *const libc::timespec) 
    };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Set event failed. Error: {:?}", err);
        return;
    }

    println!("DEBUG: Set event. count: {:?}", MAX_EVENTS);
}

fn process_events(eq: &mut EventQueue) {
    println!("DEBUG: Processing events...");

    let ten_millis = time::Duration::from_secs(10);
    thread::sleep(ten_millis);

    loop {
        let event_count = unsafe { 
            // libc::kevent(eq.queue, eq.changes.as_ptr(), eq.index as i32, eq.events.as_mut_ptr() as *mut libc::kevent, MAX_EVENTS as i32, ptr::null_mut()) 
            libc::kevent(eq.queue, ptr::null(), 0, eq.events.as_mut_ptr() as *mut libc::kevent, MAX_EVENTS as i32, ptr::null_mut()) 
        };
        if event_count == -1 {
            let err = Error::last_os_error();
            println!("DEBUG: Receive failed: {:?}", err);
            return;
        }
        println!("DEBUG: New events count: {:?}", event_count);

        eq.index = 0;

        for i in 0..event_count {
            let ev = eq.events[i as usize];
            println!("DEBUG: A new event. filter: {:?}, data: {:?}", ev.filter, ev.data);

            if ev.ident == eq.fd as usize {
                println!("DEBUG: New accept ---------------------------------------");
                on_accept(eq, &ev);
                continue;
            }

            match ev.filter {
                libc::EVFILT_READ => on_read(eq, &ev),
                libc::EVFILT_WRITE => on_write(eq, &ev),
                _ => println!("DEBUG: Not match request. data: {:?}, filter: {:?}", ev.data, ev.filter),
            }
        }
    }
}

fn on_accept(eq: &mut EventQueue, ev: &libc::kevent) {
    let mut client_addr: libc::sockaddr = unsafe { mem::uninitialized() };
    let mut client_len = mem::size_of::<libc::sockaddr>() as libc::socklen_t;

    let client_fd = unsafe { 
        libc::accept(eq.fd, &mut client_addr as *mut libc::sockaddr, &mut client_len as *mut libc::socklen_t) 
    };
    if client_fd == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: Accept failed. Error: {:?}", err);
        return;
    }
    socket_nonblocking(client_fd);

    eq.accept_total += 1;

    let client_std_addr = to_std_socket_addr(&client_addr);
    println!("DEBUG: Accept. IP: {:?}, filter: {:?}, total: {:?}", client_std_addr, ev.filter, eq.accept_total);

    set_event(eq, client_fd, libc::EVFILT_READ, libc::EV_ADD|libc::EV_ENABLE, ptr::null_mut());
}

fn on_read(eq: &mut EventQueue, ev: &libc::kevent) {
    // TODO: need read all data
    let mut buf: [u8; MAX_BUFFER] = unsafe { mem::uninitialized() };

    let recv_len = unsafe { libc::recv(ev.ident as i32, buf.as_mut_ptr() as *mut libc::c_void, MAX_BUFFER, 0) };
    if recv_len < 0 {
        let err = Error::last_os_error();
        println!("DEBUG: Read failed. Recv lent: {:?}, Error: {:?}", recv_len, err);
        return;
    }
    if recv_len == 0 {
        let err = Error::last_os_error();
        println!("DEBUG: Read failed. Recv lent: {:?}, Error: {:?}", recv_len, err);
        close_socket(ev.ident as i32);
        return;
    }
    let content_buf = &buf[0..recv_len as usize];

    println!("DEBUG: Read copy count: {:?}", recv_len);
    println!("DEBUG: Read context: {:?}", String::from_utf8_lossy(content_buf));

    set_event(eq, ev.ident as i32, libc::EVFILT_WRITE, libc::EV_ADD|libc::EV_ENABLE, ptr::null_mut());
}

fn on_write(eq: &mut EventQueue, ev: &libc::kevent) {
    let welcome_str = "Welcome to web server";
    println!("DEBUG: Send: {:?}", welcome_str);

    let buf = welcome_str.as_bytes();
    let send_len = unsafe { 
        libc::send(ev.ident as i32, buf as *const _ as *const libc::c_void, buf.len(), 0) 
    };
    if send_len < 0 {
        let err = Error::last_os_error();
        println!("DEBUG: Send buf failed. Send len: {:?}, Error: {:?}", send_len, err);
        close_socket(ev.ident as i32);
        return;
    }

    println!("DEBUG: Send len: {:?}", send_len);
    close_socket(ev.ident as i32);
}

/* ---------------------------------------------------
Establishes a user event identified by ident which is
not associated with any kernel mechanism but is trig-gered by user level code. 
The lower 24 bits of the fflags	may be used for	user defined flags
*/
fn event_notify_init(event_fd: i32) {
    let notify_kev = libc::kevent {
        ident: 0,
        filter: libc::EVFILT_USER,
        flags: libc::EV_ADD|libc::EV_CLEAR,
        fflags: 0,
        data: 0,
        udata: 0 as *mut libc::c_void,
    };

    let ret = unsafe { libc::kevent(event_fd, &notify_kev, 1, ptr::null_mut(), 0, ptr::null_mut()) };
    if ret == -1 {
        let err = Error::last_os_error();
        println!("DEBUG: kevent(EVFILT_USER, EV_ADD) failed. Error: {:?}", err);
        return;
    }
    
    println!("DEBUG: Notify event init");
}