use libc;
use std::{mem, net, ptr};
use std::io::{Error, Result};
use std::cell::RefCell;
use std::ops::Deref;

use Event;
use socket;

pub struct Connection {
    pub client_fd: i32,
    pub client_addr: net::SocketAddr,
    pub client_addr_len: u32,
    pub accept_total: usize,

    pub read_size: usize,
    pub write_size: usize,

    pub read_buf: Vec<u8>,
    pub write_buf: Vec<u8>,
}

const WEB_WELCOME: &str = "HTTP/1.1 200 OK
Date: Tue, 31 Oct 2017 13:40:35 GMT
Content-Type: text/html; charset=utf-8

<h1>Welcome to Nest!</h1>";

impl Connection {
     
    pub fn new() -> Connection {
        Connection {
             client_fd: -1,
             client_addr: unsafe { mem::uninitialized() },
             client_addr_len: 0,

             accept_total: 0,
             read_size: 0,
             write_size: 0,
             read_buf: Vec::new(),
             write_buf: WEB_WELCOME.as_bytes().to_vec(),
        }
    }

    pub fn event_accept(e: &mut Event, ev: &libc::kevent) {
        let mut count = ev.data;

        println!("DEBUG: Have {:?} need accept", count);

        for index in 0..count {
            
            let mut client_addr: libc::sockaddr = unsafe { mem::uninitialized() };
            let mut client_len = mem::size_of::<libc::sockaddr>() as libc::socklen_t;

            let client_fd = unsafe {
                libc::accept(
                    e.local_sock_fd, 
                    &mut client_addr as *mut libc::sockaddr, 
                    &mut client_len as *mut libc::socklen_t
                )
            };

            if client_fd == -1 {
                let err = Error::last_os_error();
                let errno = err.raw_os_error();

                println!("DEBUG: Accept failed. Error: {:?}", err);

                match errno.unwrap() {
                    libc::EINTR => {
                        // The accept() operation was interrupted.
                        // need retry
                        println!("ERROR: accept() interrupted. Index: {:?}", index);

                        return;
                    },
                    libc::EMFILE | libc::ENFILE => {
                        // The per-process descriptor table is full.
                        // The system file table is full.
                        // set_event(eq, eq.listen, libc::EVFILT_READ, libc::EV_DELETE|libc::EV_DISABLE, ptr::null_mut());
                        
                        //let listen_fd = c.queue.local_sock_fd;
                        //c.queue.set(listen_fd, libc::EVFILT_READ, libc::EV_ADD|libc::EV_DISABLE, ptr::null_mut());

                        println!("ERROR: File table is full. Delete with disable read event. Index: {:?}", index);
                        return;
                    },
                    libc::EAGAIN => {
                        // EWOULDBLOCK = EAGAIN in libc
                        // The socket is marked non-blocking and no connections are present to be accepted.
                        println!("ERROR: accept() not ready. Index: {:?}", index);

                        return;
                    },
                    libc::ECONNABORTED => {
                        // A connection arrived, but it was closed	while waiting on the listen queue.
                        println!("ERROR: Client closed. Index: {:?}", index);
                        
                        return;
                    },
                    _ => println!("DEBUG: Not match errno. Index: {:?}", index),
                }
                
                return;
            }
            socket::nonblocking(client_fd);

            let c = Box::new(Connection {
                client_fd: client_fd,
                client_addr: socket::to_std_socket_addr(&client_addr),
                client_addr_len: client_len,

                accept_total: index as usize,
                read_size: 0,
                write_size: 0,
                read_buf: Vec::new(),
                write_buf: "Welcome >>>>>>>>>>>>>>> to server".as_bytes().to_vec(),
            });

            println!("DEBUG: Accept. IP: {:?}, Index: {:?}, fd: {:?}", c.client_addr, c.accept_total, client_fd);

            //let raw_ptr_c: *mut _ = &mut *c;
            let raw_ptr_c = Box::into_raw(c);
            e.set(client_fd, libc::EVFILT_READ, libc::EV_ADD|libc::EV_ENABLE, raw_ptr_c as *mut libc::c_void);

            count -= 1;
        }
    }

    pub fn event_read(e: &mut Event, ev: &libc::kevent) {
        let mut c = unsafe { Box::from_raw(ev.udata as *mut Connection) };

        let fd = ev.ident as i32;
        let mut readable_len = ev.data;

        println!("DEBUG: read event. readability count: {:?}, fd: {:?}", readable_len, fd);

        if readable_len == 0 {
            return;
        }

        while readable_len > 0 {

            let mut buf: [u8; super::MAX_BUFFER] = unsafe { mem::uninitialized() };
            let recv_len = unsafe { libc::recv(fd, buf.as_mut_ptr() as *mut libc::c_void, super::MAX_BUFFER, 0) };

            if recv_len == 0 {
                // client closed.
                let err = Error::last_os_error();
                println!("DEBUG: recv() failed. client closed. error: {:?}", err);
                
                socket::close(fd);
                return;
            } 
            else if recv_len < 0 { // recv_len == -1
                let err = Error::last_os_error();
                let errno = err.raw_os_error();

                println!("DEBUG: recv() faild. error: {:?}", err);

                match errno.unwrap() {
                    libc::EAGAIN | libc::EINTR=> {
                        /*
                            The socket is marked non-blocking and the receive
                            operation would	block, or a receive timeout had	been
                            set and	the timeout expired before data	were received.

                            EINTR: The receive was interrupted by delivery of a signal before any data were available.

                            try again
                        */
                        println!("DEBUG: recv() not ready. error: {:?}", err);

                        continue;
                    },

                    libc::ECONNRESET => {
                        /*
                            ECONNRESET: The remote socket end is forcibly closed.
                        */
                        println!("DEBUG: recv() failed. remote socket closed. error: {:?}", err);
                
                        socket::close(fd);

                        return;
                    },

                    _ => {
                        println!("DEBUG: recv() failed. not match error.");

                        socket::close(fd);

                        return;
                    }
                }
            }

            readable_len -= recv_len;
            
            c.read_size += recv_len as usize;
            c.read_buf.extend_from_slice(&buf[0..recv_len as usize]);

            let str_buf = &buf[0..recv_len as usize];

            println!("DEBUG: read count: {:?}", recv_len);
            println!("DEBUG: read context: {:?}", String::from_utf8_lossy(str_buf));

            if readable_len <= 0 || recv_len < super::MAX_BUFFER as isize {
                // read completed

                println!("DEBUG: read completed. len: {:?}, full data: \n{:?}", c.read_size, String::from_utf8_lossy(c.read_buf.as_slice()));

                let raw_ptr_c = Box::into_raw(c);
                e.set(fd, libc::EVFILT_WRITE, libc::EV_ADD|libc::EV_ENABLE, raw_ptr_c as *mut libc::c_void);

                return;
            }
        }
    }

    pub fn event_write(ev: &libc::kevent) {
        let mut c = unsafe { Box::from_raw(ev.udata as *mut Connection) };

        let fd = ev.ident as i32;
        let writable_len = ev.data;
        let buf = WEB_WELCOME.as_bytes();

        println!("DEBUG: wite event. writable count: {:?}, fd: {:?}", writable_len, fd);

        while c.write_size < buf.len() {
            let mut should_len = writable_len as usize;
            let last_len = buf.len() - c.write_size;
            if should_len >= last_len {
                should_len = last_len;
            }

            let n = unsafe { 
                libc::send(fd, &buf[c.write_size..should_len] as *const _ as *const libc::c_void, should_len, 0) 
            };

            if n == 0 {
                // send() returned zero.
                let err = Error::last_os_error();
                println!("ERROR: send() returned zero. Error: {:?}", err);

                continue;
            }
            else if n < 0 {
                let err = Error::last_os_error();
                let errno = err.raw_os_error();

                match errno.unwrap() {
                    libc::EAGAIN => {
                        // The socket is marked non-blocking and the requested operation would block.
                        // try again
                        //c.queue.set(fd, libc::EVFILT_WRITE, libc::EV_ADD|libc::EV_ENABLE, ptr::null_mut());

                        println!("ERROR: send() not ready. Error: {:?}", err);
                        continue;
                    },
                    libc::ENOBUFS => {
                        /*
                            ENOBUFS: The system was unable to allocate an internal buffer. 
                                    The operation may succeed when buffers become avail-able.
                            ENOBUFS: The output queue for a network interface was full.
                                    This generally indicates that the interface has
                                    stopped sending, but may be caused by transient con-gestion.

                            try again.
                        */
                        //c.queue.set(fd, libc::EVFILT_WRITE, libc::EV_ADD|libc::EV_ENABLE, ptr::null_mut());

                        println!("ERROR: send() not ready. Error: {:?}", err);
                        socket::close(fd);
                        return;
                    },
                    libc::EHOSTUNREACH | libc::EHOSTDOWN | libc::ENETDOWN => {
                        /*
                            EHOSTUNREACH: The remote host was unreachable.
                            EHOSTDOWN: The remote host was down.
                            ENETDOWN: The remote network was down.
                        */
                        
                        println!("ERROR: send() remote closed. Error: {:?}", err);
                        socket::close(fd);
                        return;
                    },
                    libc::EISCONN => {
                        // A destination address was specified and	the socket is already connected.
                        // try again.
                        //c.queue.set(fd, libc::EVFILT_WRITE, libc::EV_ADD|libc::EV_ENABLE, ptr::null_mut());

                        println!("ERROR: send() not ready. Error: {:?}", err);
                        socket::close(fd);
                        continue;
                    },
                    libc::ECONNREFUSED => {
                        /*
                            The socket received an ICMP destination	unreachable
                            message	from the last message sent. This typically
                            means that the receiver	is not listening on the	remote
                            port.
                        */
                        
                        println!("ERROR: send() can not to target. Error: {:?}", err);
                        socket::close(fd);
                        return;
                    },
                    libc::EPIPE => {
                        /*
                            The socket is unable to	send anymore data (SBS_CANTSENDMORE has been set on the socket). 
                            This typically means that the socket is not connected.

                            { code: 32, message: "Broken pipe" }
                        */
                        println!("ERROR: send() -> EPIPE, the socket is not connected. Error: {:?}", err);
                        socket::close(fd);
                        return;
                    }
                    _ => {
                        println!("ERROR: send() failed not match. Error: {:?}", err);
                        socket::close(fd);
                        return;
                    },
                }
            }
            c.write_size += n as usize;

            println!("DEBUG: write count: {:?}", n);
            println!("DEBUG: write context: {:?}", String::from_utf8_lossy(&buf[c.write_size..should_len]));

            if c.write_size >= c.write_buf.len() {
                socket::close(fd);

                return;
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("DEBUG: Connection drop. fd: {:?}", self.client_fd);
        let _ = unsafe { libc::close(self.client_fd) };
    }
}