// TODO: Remove
#![allow(dead_code)]
#![allow(unused_imports)]

mod libc {
    extern crate libc;
    pub use self::libc::*;
}

mod winapi {
    extern crate winapi;
    pub use self::winapi::*;
}

#[macro_use]
extern crate log;

#[macro_use]
mod macros;


// TODO: Remove pub
pub mod core;
pub mod os;
pub mod event;

pub use self::macros::*;
pub use self::core::{NsResult, NsError, Nest, Config};

cfg_if! {
    if #[cfg(any(target_os = "linux",
                 target_os = "android",
                 target_os = "emscripten",
                 target_os = "fuchsia"))] {
    } else if #[cfg(any(target_os = "macos",
                        target_os = "ios",
                        target_os = "freebsd",
                        target_os = "dragonfly",
                        target_os = "openbsd",
                        target_os = "netbsd",
                        target_os = "bitrig"))] {
       
    } else if #[cfg(target_os = "solaris")] {
       
    } else {
        // Unknown target_os
    }
}