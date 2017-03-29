
mod libc {
    extern crate libc;
    pub use self::libc::*;
}

pub mod os;
