use std::io;
use std::ffi;
use std::fmt;
use std::result;
use std::time;


#[derive(Debug)]
pub enum Error {
    Empty,
    Io(io::Error),
    Serde(serde_json::Error),
    SystemTime(time::SystemTimeError),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) =>
                write!(f, "{}", error),
            Error::Empty =>
                write!(f, "Error"),
            Error::Serde(ref error) =>
                write!(f, "{}", error),
            Error::SystemTime(ref error) =>
                write!(f, "{}", error),
        }
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}


impl From<ffi::OsString> for Error {
    fn from(_err: ffi::OsString) -> Error {
        Error::Empty
    }
}


impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}


impl From<time::SystemTimeError> for Error {
    fn from(err: time::SystemTimeError) -> Error {
        Error::SystemTime(err)
    }
}


pub type Result<T> = result::Result<T, Error>;
