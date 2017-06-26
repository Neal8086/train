use std::{mem, ptr};
use libc;
use os::*;
use NsError;
use NsResult;
use event::traits::NsEventTrait;


pub struct NsKqueue {
    pub fd: ns_fd,
}


impl NsEventTrait for NsKqueue {

    fn new() -> NsResult<NsKqueue> {
        let ret = unsafe { libc::kqueue() };

        if ret == -1 {
            println!("DEBUG: Create kqueue failed. {:?}", ret);
            return Err(NsError::Unknow);
        }

        Ok(NsKqueue{ fd: ret })
    }

    fn add_event(&self) {}

    fn del_event(&self) {}

    fn notify_init(&self) -> NsResult<i32> {

        let notify_kev = ns_kevent {
            ident: 0,
            filter: NS_EVFILT_USER,
            flags: NS_EV_ADD | NS_EV_CLEAR,
            fflags: 0,
            data: 0,
            udata: 0 as *mut ns_void,
        }; 

        let ret = unsafe { libc::kevent(self.fd, &notify_kev, 1, ptr::null_mut(), 0, ptr::null_mut()) };
        if ret == -1 {
            println!("DEBUG: Notify kqueue init failed. {:?}", ret);
            return Err(NsError::Unknow);
        }

        Ok(0)
    }

    fn notify(&self) {}
    
    fn process_events(&self) {}
}

impl Drop for NsKqueue {
    fn drop(&mut self) {
        println!("DEBUG: Close FD, {}", self.fd);

        let ret = unsafe { libc::close(self.fd) };
        if ret == -1 {
            println!("DEBUG: Close kqueue failed. {:?}", ret);
        }

        self.fd = -1;
        println!("DEBUG: FD Closed: {}", self.fd);
    }
}