use libc;
use std;
use std::{mem, ptr, thread, time};
use std::io::{Error};

use global;

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
struct Sigaction {
    pub signo: libc::c_int,
    pub name: &'static str,
    pub action_name: &'static str,
    pub handler: libc::sighandler_t,
}

const SIGNALS : [Sigaction; 12] = [
    Sigaction {
        signo: libc::SIGHUP,
        name: "SIGHUP",
        action_name: "reload",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGINFO,
        name: "SIGINFO",
        action_name: "reopen", // if linux thread use SIGUSR1
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGWINCH,
        name: "SIGWINCH",
        action_name: "no accept",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGTERM,
        name: "SIGTERM",
        action_name: "stop",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGQUIT,
        name: "SIGQUIT",
        action_name: "quit",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGXCPU,
        name: "SIGXCPU",
        action_name: "change bin", // if linux threads use SIGUSR2
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGALRM,
        name: "SIGALRM",
        action_name: "SIGALRM",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGINT,
        name: "SIGINT",
        action_name: "SIGINT",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGIO,
        name: "SIGIO",
        action_name: "SIGIO",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGCHLD,
        name: "SIGCHLD",
        action_name: "SIGCHLD",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGSYS,
        name: "SIGSYS, SIG_IGN",
        action_name: "SIGSYS, SIG_IGN",
        handler: libc::SIG_IGN,
    },
    Sigaction {
        signo: libc::SIGPIPE,
        name: "SIGPIPE, SIG_IGN",
        action_name: "SIGPIPE, SIG_IGN",
        handler: libc::SIG_IGN,
    }
];

pub fn set_empty_signal(sigset: &mut libc::sigset_t) {
    let ret = unsafe { libc::sigemptyset(sigset as *mut libc::sigset_t) };
    if ret == -1 {
        println!("DEBUG: sigemptyset failed. Error: {:?}", Error::last_os_error());

        std::process::exit(1);
    }
}

pub fn add_signal(sigset: &mut libc::sigset_t, signum: libc::c_int) {
    let ret = unsafe { libc::sigaddset(sigset as *mut libc::sigset_t, signum) };
    if ret == -1 {
        println!("DEBUG: sigaddset failed. Error: {:?}", Error::last_os_error());

        std::process::exit(1);
    }
}

pub fn signal_set_block(sigset: &mut libc::sigset_t) {
    let ret = unsafe { sigprocmask(libc::SIG_BLOCK, sigset as *const libc::sigset_t, ptr::null()) };

    if ret == -1 {
        println!("DEBUG: sigprocmask failed. Error: {:?}", Error::last_os_error());

        std::process::exit(1);
    }
}

pub fn signal_set_mask(sigset: &mut libc::sigset_t) {
    let ret = unsafe { sigprocmask(libc::SIG_SETMASK, sigset as *const libc::sigset_t, ptr::null()) };

    if ret == -1 {
        println!("DEBUG: sigprocmask failed. Error: {:?}", Error::last_os_error());

        std::process::exit(1);
    }
}

pub fn signal_suspend(sigset: &mut libc::sigset_t) {
    let ret = unsafe { sigsuspend(sigset as *mut libc::sigset_t) };

    if ret == -1 {
        println!("DEBUG: sigsuspend failed. PID: {:?}, Error: {:?}", unsafe{ libc::getpid() }, Error::last_os_error());

        //std::process::exit(1);
    }
}

pub fn signal_handler(signo: usize, siginfo: libc::siginfo_t, ucontext: libc::c_void) {
    println!("DEBUG: signal handler, signo: {:?}, PID: {:?}", signo, unsafe { libc::getpid() });
    
    let is_master = unsafe { global::IS_MASTER };

    for sig in SIGNALS.iter() {
        if signo as i32 == sig.signo {
            println!("INFO: receive signal is {:?}({:?}), self PID: {:?}, is master: {:?}", 
                sig.name, 
                sig.signo, 
                unsafe { libc::getpid() }, 
                is_master);
        }
    
        if is_master && signo as i32 == libc::SIGCHLD {
            std::process::exit(0);
        } else if !is_master {
            std::process::exit(0);
        }
    }
}

pub fn init() {

    for sig in SIGNALS.iter() {
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

