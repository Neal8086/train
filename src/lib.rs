
mod libc {
    extern crate libc;
    pub use self::libc::*;
}

mod winapi {
    extern crate winapi;
    pub use self::winapi::*;
}

pub mod os;
