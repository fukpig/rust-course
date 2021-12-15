//use std::{error, fmt, io, result};
use std::{fmt, result};

pub enum Error {
    System(String),
    NotFound(String),
    AlreadyExist(String),

    //Io(io::Error), //wait for tcp thermometer
    //Message(serde_json::Error), //wait for tcp connect to thermometer
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::System(err) => write!(f, "system error: {}", err),
            Error::NotFound(err) => write!(f, "not Found error: {}", err),
            Error::AlreadyExist(err) => write!(f, "device already exist error: {}", err),
            /*Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Message(ref err) => write!(f, "Invalid message: {}", err),*/
        }
    }
}

//impl error::Error for Error {}

/*impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Message(err)
    }
}*/

pub type Result<T> = result::Result<T, Error>;
