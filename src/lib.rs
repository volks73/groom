//! # Groom
//!
//! Groom is primarily a Command-Line Interface (CLI) application for processing
//! [mustache](https://mustache.github.io/) templates.

#[macro_use] extern crate log;

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

pub use self::groom::Groom;

mod groom;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Input(String),
    Io(io::Error)
}

impl Error {
    pub fn code(&self) -> i32 {
        match *self{
            Error::Generic(..) => 1,
            Error::Input(..) => 2,
            Error::Io(..) => 3,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Generic(ref msg) => write!(f, "{}", msg),
            Error::Input(ref msg) => write!(f, "{}", msg),
            Error::Io(ref err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Generic(..) => "Generic",
            Error::Input(..) => "Input",
            Error::Io(..) => "IO",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

