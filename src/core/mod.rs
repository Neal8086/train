use std;

pub mod error;
pub mod config;
pub mod nest;


pub use self::config::*;
pub use self::nest::*;
pub use self::error::NsError;

pub type NsResult<T> = std::result::Result<T, NsError>;