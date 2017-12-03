
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate libc;

mod signal;

use std::{mem, thread, time};
use std::io::{Error};


fn main() {
    let mut child_pids: Vec<i32> = Vec::new();

    println!("INFO: multi-Process test");
    println!("INFO: self PID: {:?}", unsafe { libc::getpid() });


    println!("INFO: begin init signals.");
    signal::init();

    let mut sigset: libc::sigset_t = unsafe { mem::uninitialized() };

    signal::set_empty_signal(&mut sigset);
    signal::add_signal(&mut sigset, libc::SIGCHLD);
    signal::add_signal(&mut sigset, libc::SIGALRM);
    signal::add_signal(&mut sigset, libc::SIGIO);
    signal::add_signal(&mut sigset, libc::SIGINT);
    signal::add_signal(&mut sigset, libc::SIGHUP);
    signal::add_signal(&mut sigset, libc::SIGINFO);
    signal::add_signal(&mut sigset, libc::SIGWINCH);
    signal::add_signal(&mut sigset, libc::SIGTERM);
    signal::add_signal(&mut sigset, libc::SIGQUIT);
    signal::add_signal(&mut sigset, libc::SIGXCPU);

    signal::signal_proc_mask(&mut sigset);

    signal::set_empty_signal(&mut sigset);

    println!("INFO: begin fork process.");
    
    for _ in 1..3 {        
        let pid = unsafe { libc::fork() };
        if pid < 0 {
            println!("ERROR: fork error: {:?}", Error::last_os_error());
            return;
        }

        if pid == 0 {
            let self_pid = unsafe{ libc::getpid()};
            let parent_pid = unsafe{ libc::getppid() };

            println!("Child process: pid: {:?}, ppid: {:?}", self_pid, parent_pid);
            child_pids.push(self_pid);

            println!("DEBUG: store childs: {:?}", child_pids);

            let dura = time::Duration::from_secs(10);
            thread::sleep(dura);

            return;
        } else {
            println!("Master process, PID: {:?}, new fork child PID: {:?}", unsafe{ libc::getpid()}, pid);
        }
    }
    println!("INFO: end fork process.");

    for i in child_pids {
        println!("DEBUG: store pid: {:?}", i);
    }


    loop {
        println!("INFO: loop signal suspend.");

        signal::signal_suspend(&mut sigset);

        println!("INFO: proccess received a signal and exit");
        break;
    }

}
