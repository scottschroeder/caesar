use std::error;
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidConfig(String),
    NotChar(String),
    InvalidToml,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidConfig(ref err) => write!(f, "InvalidConfig: {}", err),
            Error::NotChar(ref err) => write!(f, "NotChar: {}", err),
            Error::InvalidToml => write!(f, "String is not valid TOML"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidConfig(_) => "could not parse config as Encoding",
            Error::NotChar(_) => "string was not a single char",
            Error::InvalidToml => "string was not valid TOML",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidConfig(_) => None,
            Error::NotChar(_) => None,
            Error::InvalidToml => None,
        }
    }
}
