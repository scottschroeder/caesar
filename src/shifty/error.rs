use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidConfig(String),
    NotChar(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidConfig(ref err) => write!(f, "InvalidConfig: {}", err),
            Error::NotChar(ref err) => write!(f, "NotChar: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidConfig(_) => "could not parse config as Encoding",
            Error::NotChar(_) => "string was not a single char",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidConfig(_) => None,
            Error::NotChar(_) => None,
        }
    }
}
