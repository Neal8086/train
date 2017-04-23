
#[cfg(unix)]
#[path = "unix/mod.rs"]
mod os;

#[cfg(windows)]
#[path = "win/mod.rs"]
mod os;

mod addr;

pub use self::os::*;
pub use self::addr::*;