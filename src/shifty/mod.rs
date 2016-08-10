
use std;

mod error;
mod encoding;
mod parser;
mod encoders;

pub use self::encoding::Encoding;
pub use self::encoding::Action;
pub use self::error::Error;
pub type Result<T> = std::result::Result<T, self::error::Error>;


pub use self::encoders::*;
