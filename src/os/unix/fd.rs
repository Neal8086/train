use libc::{c_int, close};
use std::{fmt};

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

impl fmt::Display for NsFd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_raw_fd())
    }
}

impl fmt::Debug for NsFd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
