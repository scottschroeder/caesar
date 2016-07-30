use std;

mod error;
mod encoding;

pub use self::encoding::Encoding;
pub use self::error::Error;



pub type Result<T> = std::result::Result<T, self::error::Error>;
