use libc;
use std::result::Result;


pub fn socketpair() -> Result<()> {
    let mut fds = [-1, -1];
    let ret = unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
    if ret == -1 {
        println!("DEBUG: Create socketpair failed! {:?}", ret);
        return Err(NsError::Unknow);
    }

    Ok((fds[0], fds[1]))
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