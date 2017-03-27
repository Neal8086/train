
#[cfg(unix)]
#[path = "unix/mod.rs"]
mod os;

#[cfg(windows)]
#[path = "win/mod.rs"]
mod os;

pub use self::os::*;
