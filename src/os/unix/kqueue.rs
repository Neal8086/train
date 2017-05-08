use std::{mem};
use libc;
use super::*;


pub fn ns_kqueue() -> NsResult<ns_fd> {
    let ret = unsafe { libc::kqueue(); };

    if ret == -1 {
        println!("DEBUG: Create kqueue failed. {:?}", ret);
        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn ns_kevent(kq: ns_fd, changelist: &Vec<ns_kevent>, nchanges: ns_int, 
                events: &mut Vec<ns_kevent>, nevents: ns_int, timeout: &ns_timespec) 
                -> NsResult<ns_int> {
                
    Ok(0)
}