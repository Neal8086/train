use libc::{c_int, close};


pub struct NsFd {
    fd: c_int,
}

impl NsFd {
    pub fn new(fd: c_int) -> NsFd {
        NsFd { fd: fd }
    }

    pub fn as_raw_fd(&self) -> c_int {
        self.fd
    }
}


impl Drop for NsFd {
    fn drop(&mut self) {
        let _ = unsafe { close(self.fd) };
    }
}
