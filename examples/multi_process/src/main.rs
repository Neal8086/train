
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate libc;

mod global;
mod signal;
mod socketpair;

use std::{fmt, mem, thread, time};
use std::io::{Error};


pub struct Context {
    master_pid: i32,
    childs: Vec<i32>,
    channels: Vec<(i32, i32)>,
}

fn main() {
    println!("INFO: multi-Process test");

    let mut ctx = Context{
        master_pid: unsafe { libc::getpid() },
        childs: Vec::new(),
        channels: Vec::new(),
    };
    
    println!("INFO: self PID: {:?}", ctx.master_pid);


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

    signal::signal_set_block(&mut sigset);
    signal::set_empty_signal(&mut sigset);


    start_worker_processes(&mut ctx);

    let self_pid = unsafe { libc::getpid() };
    loop {
        println!("INFO: loop signal suspend. self PID: {:?}", self_pid);

        signal::signal_suspend(&mut sigset);

        println!("INFO: proccess received a signal. NO: {:?}, PID: {:?}", sigset, self_pid);
    }

}

fn get_socketpair(pid: libc::c_int) -> (libc::c_int, libc::c_int) {
    let ret = socketpair::socketpair().unwrap();

    println!("DEBUG: new socket pair: {:?}", ret);

    socketpair::nonblocking(ret.0);
    socketpair::nonblocking(ret.1);

    socketpair::fio_async(ret.0, true);
    socketpair::fcntl_set(ret.0, libc::F_SETOWN, pid);

    socketpair::fcntl_set(ret.0, libc::F_SETFD, libc::FD_CLOEXEC);
    socketpair::fcntl_set(ret.1, libc::F_SETFD, libc::FD_CLOEXEC);

    return ret;
}

fn start_worker_processes(ctx: &mut Context) {
    println!("INFO: start_worker_processes.");

    for _ in 1..3 {
        let channel = get_socketpair(ctx.master_pid);
        ctx.channels.push(channel);

        let pid = unsafe { libc::fork() };

        match pid {
            -1 => println!("ERROR: fork error: {:?}", Error::last_os_error()),

            0 => {
                woker();
                return;
            },

            _ => {                
                println!("INFO: Master process, PID: {:?}, new fork child PID: {:?}", unsafe{ libc::getpid()}, pid);
                ctx.childs.push(pid);
            },
        }
    }

    for pid in ctx.childs.clone() {
        println!("DEBUG: childs pid: {:?}", pid);
    }
}

fn woker() {
    let self_pid = unsafe{ libc::getpid()};
    let parent_pid = unsafe{ libc::getppid() };
    println!("INFO: Worker process: pid: {:?}, ppid: {:?}", self_pid, parent_pid);

    /*
    let mut sigset: libc::sigset_t = unsafe { mem::uninitialized() };
    signal::set_empty_signal(&mut sigset);
    signal::signal_set_mask(&mut sigset);
    */

    unsafe {
        global::IS_MASTER = false
    };

    let dura = time::Duration::from_secs(10);
    loop {
        thread::sleep(dura);

        println!("DEBUG: Online PID: {:?}", self_pid);
    }
    /*
    let send_str = format!("From PID: {}!", self_pid);;
    socketpair::send(ctx.channels.0, &send_str);
    */
}


