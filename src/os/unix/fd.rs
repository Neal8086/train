use libc;
use NsError;
use NsResult;
use super::*;


pub fn ns_flags(fd: ns_fd) -> NsResult<ns_int> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };
    if flags < 0 {
        print!("DEBUG: Can not get fd flag: {:?}", flags);
        return Err(NsError::Unknow);
    }
    
    Ok(flags)
}

