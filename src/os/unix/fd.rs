use libc::{self, c_int};
use NsError;
use NsResult;


pub fn ns_flags(fd: c_int) -> NsResult<c_int> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };
    if flags < 0 {
        print!("DEBUG: Can not get fd flag: {:?}", flags);
        return Err(NsError::Unknow);
    }
    
    Ok(flags)
}

pub fn ns_set_nonblocking(fd: c_int) -> NsResult<c_int> {
    let mut flags = ns_flags(fd).unwrap();
    flags |= libc::O_NONBLOCK;

    let ret = unsafe { libc::fcntl(fd, libc::F_SETFL, flags) };
    if ret == -1 {
        println!("DEBUG: Set Non-blocking FD failed");
        return Err(NsError::Unknow);
    }

    Ok(0)
}