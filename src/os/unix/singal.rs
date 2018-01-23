use libc;

extern "C" {
    fn sigprocmask(signum: libc::c_int, set: *const libc::sigset_t, oldset: *const libc::sigset_t) -> libc::c_int;
    fn sigsuspend(set: *mut libc::sigset_t) -> libc::c_int;
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: libc::sighandler_t,
    pub sa_handler: libc::sighandler_t,
    pub sa_mask: libc::sigset_t,
    pub sa_flags: libc::c_int,
}

#[derive(Copy, Clone)]
struct NsSigaction {
    pub signo: libc::c_int,
    pub name: &'static str,
    pub action_name: &'static str,
    pub handler: libc::sighandler_t,
}