use libc::c_int;


#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
unsafe fn os_errno() -> *mut c_int {
    extern { fn __error() -> *mut c_int; }
    __error()
}

#[cfg(target_os = "dragonfly")]
unsafe fn os_errno() -> *mut c_int {
    extern { fn __dfly_error() -> *mut c_int; }
    __dfly_error()
}

#[cfg(any(target_os = "openbsd", target_os = "netbsd"))]
unsafe fn os_errno() -> *mut c_int {
    extern { fn __errno() -> *mut c_int; }
    __errno()
}

#[cfg(any(target_os = "linux", target_os = "android"))]
unsafe fn os_errno() -> *mut c_int {
    extern { fn __errno_location() -> *mut c_int; }
    __errno_location()
}

unsafe fn clear() -> () {
    *os_errno() = 0;
}

pub struct NsOsErrno {
    
}
