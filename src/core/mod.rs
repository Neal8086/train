use std;

pub mod error;


pub use self::error::NsError;
pub type NsResult<T> = std::result::Result<T, NsError>;