use std::{mem};
use libc;
use os::*;
use NsError;
use NsResult;
use event::traits::NsEventTrait;


pub fn ns_kqueue() -> NsResult<ns_fd> {
    let ret = unsafe { libc::kqueue() };

    if ret == -1 {
        println!("DEBUG: Create kqueue failed. {:?}", ret);
        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_kevent(kq: ns_fd, 
                change_list: *const ns_kevent, 
                n_changes: ns_int, 
                event_list: *mut ns_kevent, 
                n_events: ns_int, 
                timeout: *const ns_timespec) 
                -> NsResult<ns_int> {
    
    let ret = unsafe { libc::kevent(kq, change_list, n_changes, event_list, n_events, timeout) };
    if ret == -1 {
        println!("DEBUG: Create kevent Failed.");
        return Err(NsError::Unknow);
    }

    Ok(0)
}

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

    fn add_event() {}

    fn del_event() {}

    fn notify() {}
    
    fn process_events() {}
}