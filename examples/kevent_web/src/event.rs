
use libc;
use std::{mem, ptr, thread, time};
use std::io::{Error, Result};
use connection::Connection;

const MAX_EVENTS: usize = 512;


pub struct Event {
    pub queue_fd: i32,
    pub local_sock_fd: i32,
    pub index: usize,
    pub change_list: [libc::kevent; MAX_EVENTS],
    pub event_list: [libc::kevent; MAX_EVENTS],
}

impl Event {
     
    pub fn new(fd: i32) -> Result<Event> {
        let queue_fd = unsafe { libc::kqueue() };
        if queue_fd == -1 {
            let err = Error::last_os_error();
            println!("DEBUG: Create kqueue failed. Error: {:?}", err);
            return Err(err);
        }

        Ok(
            Event {
                queue_fd: queue_fd,
                local_sock_fd: fd,
                index: 0,
                change_list: unsafe { mem::uninitialized() },
                event_list: unsafe { mem::uninitialized() },
            }
        )
    }

    pub fn init_notify(&mut self) {
        let fd = self.local_sock_fd;
        self.set(fd, libc::EVFILT_READ, libc::EV_ADD|libc::EV_ENABLE, ptr::null_mut());
    }

    pub fn set(&mut self, ident: i32, filter: i16, flags: u16, udata: *mut libc::c_void) {
        if self.index >= MAX_EVENTS {
            println!("DEBUG: Change list is filled up.");
            self.index = 0;
        }

        self.change_list[self.index] = libc::kevent {
            ident: ident as usize,
            filter: filter,
            flags: flags,
            fflags: 0,
            data: 0,
            udata: udata,
        };

        self.index += 1;
    }

    pub fn process(&mut self) -> i32 {
        let n = unsafe { 
            libc::kevent(
                self.queue_fd, 
                self.change_list.as_ptr(), 
                self.index as i32, 
                self.event_list.as_mut_ptr() as *mut libc::kevent, 
                MAX_EVENTS as i32, 
                ptr::null(),
            )
        };
        self.index = 0;
        
        if n == -1 {
            let err = Error::last_os_error();
            println!("DEBUG: Loop event failed: {:?}", err);
            return -1;
        }
        println!("DEBUG: New events count: {:?}", n);

        n
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        /*
            when the file descriptor is closed the kqueue automatically deletes 
            its filters so we do not need to delete explicitly the event 
            before the closing the file descriptor.
        */

        println!("DEBUG: Event drop. fd: {:?}", self.queue_fd);
        let _ = unsafe { libc::close(self.queue_fd) };
    }
}


