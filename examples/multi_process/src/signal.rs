use libc;
use std;
use std::{mem, ptr, thread, time};
use std::io::{Error};


extern "C" {
    fn sigprocmask(signum: libc::c_int, set: *const libc::sigset_t, oldset: *const libc::sigset_t) -> libc::c_int;
    fn sigsuspend(set: *mut libc::sigset_t) -> libc::c_int;
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

pub fn signal_proc_mask(sigset: &mut libc::sigset_t) {
    let ret = unsafe { sigprocmask(libc::SIG_BLOCK, sigset as *const libc::sigset_t, ptr::null()) };

    if ret == -1 {
        println!("DEBUG: sigprocmask failed. Error: {:?}", Error::last_os_error());

        std::process::exit(1);
    }
}

pub fn signal_suspend(sigset: &mut libc::sigset_t) {
    let ret = unsafe { sigsuspend(sigset as *mut libc::sigset_t) };

    if ret == -1 {
        println!("DEBUG: sigsuspend failed. Error: {:?}", Error::last_os_error());

        std::process::exit(1);
    }
}

pub fn signal_handler(signo: usize) {
    println!("DEBUG: signal_handler, signo: {:?}", signo);
}

pub fn init() {

    for sig in SIGNALS.iter() {
        let mut sigset = unsafe { mem::uninitialized::<libc::sigaction>() };

        set_empty_signal(&mut sigset.sa_mask);

        if sig.handler != libc::SIG_IGN {
            sigset.sa_sigaction = signal_handler as usize;
        }

        if -1 == unsafe { libc::sigaction(sig.signo, &sigset as *const libc::sigaction, ptr::null_mut()) } {
           println!("DEBUG: sigaction failed. Error: {:?}", Error::last_os_error());
        } else {
            println!("DEBUG: sigaction: {:?}", sig.name);
        }
    }
}

