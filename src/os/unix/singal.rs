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


pub const NS_SIGNALS : [NsSigaction; 12] = [
    NsSigaction {
        signo: libc::SIGHUP,
        name: "SIGHUP",
        action_name: "reload",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGINFO,
        name: "SIGINFO",
        action_name: "reopen", // if linux thread use SIGUSR1
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGWINCH,
        name: "SIGWINCH",
        action_name: "no accept",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGTERM,
        name: "SIGTERM",
        action_name: "stop",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGQUIT,
        name: "SIGQUIT",
        action_name: "quit",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGXCPU,
        name: "SIGXCPU",
        action_name: "change bin", // if linux threads use SIGUSR2
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGALRM,
        name: "SIGALRM",
        action_name: "SIGALRM",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGINT,
        name: "SIGINT",
        action_name: "SIGINT",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGIO,
        name: "SIGIO",
        action_name: "SIGIO",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGCHLD,
        name: "SIGCHLD",
        action_name: "SIGCHLD",
        handler: 0,
    },
    NsSigaction {
        signo: libc::SIGSYS,
        name: "SIGSYS, SIG_IGN",
        action_name: "SIGSYS, SIG_IGN",
        handler: libc::SIG_IGN,
    },
    NsSigaction {
        signo: libc::SIGPIPE,
        name: "SIGPIPE, SIG_IGN",
        action_name: "SIGPIPE, SIG_IGN",
        handler: libc::SIG_IGN,
    }
];

pub fn init() {
    for sig in NS_SIGNALS.iter() {
        let mut sa = unsafe { mem::uninitialized::<sigaction>() };

        if sig.handler != libc::SIG_IGN {
            sa.sa_sigaction = signal_handler as usize;
            sa.sa_flags = libc::SA_SIGINFO;
        } else {
            sa.sa_handler = libc::SIG_IGN;
        }

        set_empty_signal(&mut sa.sa_mask);

        if -1 == unsafe { libc::sigaction(sig.signo, &sa as *const _ as *const libc::sigaction, ptr::null_mut()) } {
           println!("DEBUG: sigaction failed. Error: {:?}", Error::last_os_error());
        } else {
            println!("DEBUG: sigaction: {:?}", sig.name);
        }
    }
}