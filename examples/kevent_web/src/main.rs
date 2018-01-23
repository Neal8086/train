#![allow(dead_code)]
#![allow(unused_imports)]
extern crate libc;

#[macro_use]
extern crate log;

mod event;
mod connection;
mod socket;

use std::{mem, ptr, fmt, net, thread, time};
use std::io::Error;
use std::fmt::Debug;
use log::{Level, LevelFilter, Log, Record, Metadata};


use event::Event;
use connection::Connection;

const MAX_EVENTS: usize = 512;
const MAX_BUFFER: usize = 4095;
const MAX_WRITE_SIZE: usize = 4095;
const LISTEN_BACKLOG: usize = 510;


fn main() {
    log::set_max_level(LevelFilter::Trace);
    trace!("Kevent web sample start.");
    info!(target: "kevent", "Kevent web sample start");

    let listen_fd = socket::listen(&[127, 0, 0, 1], 3000).unwrap();

    let mut e = Event::new(listen_fd).unwrap();

    event_loop(&mut e);

    socket::close(listen_fd);
}

fn event_loop(e: &mut Event) {
    let lcoal_fd = e.local_sock_fd;

    e.init_notify();

    loop {
        let n = e.process();
        
        for i in 0..n {
            let ev = e.event_list[i as usize];
            
            println!("INFO: A new event. filter: {:?}, data: {:?}", ev.filter, ev.data);

            if ev.ident as i32 == lcoal_fd {
                Connection::event_accept(e, &ev);
                break;
            }

            match ev.filter {
                libc::EVFILT_READ => {
                    Connection::event_read(e, &ev);
                },
                libc::EVFILT_WRITE => {
                    Connection::event_write(&ev);
                },
                _ => println!("DEBUG: Not match request. filter: {:?}", ev.filter),
            }
        }
    }
}

/* ---------------------------------------------------
Establishes a user event identified by ident which is
not associated with any kernel mechanism but is trig-gered by user level code. 
The lower 24 bits of the fflags	may be used for	user defined flags
*/
/*
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
*/
