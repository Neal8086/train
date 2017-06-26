use libc::{self, c_int, int16_t, uint16_t};


pub const NS_TCP_NOPUSH: c_int = 0x04;

pub const NS_EVFILT_READ: int16_t = libc::EVFILT_READ;
pub const NS_EVFILT_WRITE: int16_t = libc::EVFILT_WRITE;
pub const NS_EVFILT_AIO: int16_t = libc::EVFILT_AIO;
pub const NS_EVFILT_VNODE: int16_t = libc::EVFILT_VNODE;
pub const NS_EVFILT_PROC: int16_t = libc::EVFILT_PROC;
pub const NS_EVFILT_SIGNAL: int16_t = libc::EVFILT_SIGNAL;
pub const NS_EVFILT_TIMER: int16_t = libc::EVFILT_TIMER;
pub const NS_EVFILT_MACHPORT: int16_t = libc::EVFILT_MACHPORT;
pub const NS_EVFILT_FS: int16_t = libc::EVFILT_FS;
pub const NS_EVFILT_USER: int16_t = libc::EVFILT_USER;
pub const NS_EVFILT_VM: int16_t = libc::EVFILT_VM;

pub const NS_EV_ADD: uint16_t = libc::EV_ADD;
pub const NS_EV_DELETE: uint16_t = libc::EV_DELETE;
pub const NS_EV_ENABLE: uint16_t = libc::EV_ENABLE;
pub const NS_EV_DISABLE: uint16_t = libc::EV_DISABLE;
pub const NS_EV_ONESHOT: uint16_t = libc::EV_ONESHOT;
pub const NS_EV_CLEAR: uint16_t = libc::EV_CLEAR;
pub const NS_EV_RECEIPT: uint16_t = libc::EV_RECEIPT;
pub const NS_EV_DISPATCH: uint16_t = libc::EV_DISPATCH;
pub const NS_EV_FLAG0: uint16_t = libc::EV_FLAG0;
pub const NS_EV_POLL: uint16_t = libc::EV_POLL;
pub const NS_EV_FLAG1: uint16_t = libc::EV_FLAG1;
pub const NS_EV_OOBAND: uint16_t = libc::EV_OOBAND;
pub const NS_EV_ERROR: uint16_t = libc::EV_ERROR;
pub const NS_EV_EOF: uint16_t = libc::EV_EOF;
pub const NS_EV_SYSFLAGS: uint16_t = libc::EV_SYSFLAGS;


cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod apple;
        pub use self::apple::*;
    } else if #[cfg(any(target_os = "freebsd"))] {
        mod freebsd;
        pub use self::freebsd::*;
    } else if #[cfg(target_os = "netbsd")] {
        mod netbsd;
        pub use self::netbsd::*;
    } else if #[cfg(target_os = "dragonfly")] {
        mod dragonfly;
        pub use self::dragonfly::*;
    } else {
        // Unknown target_os
    }
}

pub type ns_kevent = libc::kevent;
pub type ns_timespec = libc::timespec;