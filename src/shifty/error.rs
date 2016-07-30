use std::error;
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidConfig(ConfigError),
    CharNotInEncoding(char),
    NumberNotInEncoding(super::encoding::EncodeNum),
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Error {
        Error::InvalidConfig(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidConfig(ref err) => write!(f, "{}", err),
            Error::CharNotInEncoding(c) => write!(f, "Char {} not in encoding", c),
            Error::NumberNotInEncoding(n) => write!(f, "Number {:?} not in encoding", n),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidConfig(_) => "could not parse config as Encoding",
            Error::CharNotInEncoding(_) => "found char not in encoding",
            Error::NumberNotInEncoding(_) => "found number not in encoding",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidConfig(ref e) => Some(e),
            Error::CharNotInEncoding(_) => None,
            Error::NumberNotInEncoding(_) => None,
        }
    }
}



#[derive(PartialEq, Eq, Debug)]
pub enum ConfigError {
    SchemaError(String),
    ValueNotChar(String),
    InvalidToml,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::SchemaError(ref err) => write!(f, "SchemaError: {}", err),
            ConfigError::ValueNotChar(ref err) => write!(f, "ValueNotChar: {}", err),
            ConfigError::InvalidToml => write!(f, "String is not valid TOML"),
        }
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::SchemaError(_) => "could not parse config as Encoding",
            ConfigError::ValueNotChar(_) => "string was not a single char",
            ConfigError::InvalidToml => "string was not valid TOML",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ConfigError::SchemaError(_) => None,
            ConfigError::ValueNotChar(_) => None,
            ConfigError::InvalidToml => None,
        }
    }
}

