// Copyright (C) 2017 Christopher R. Field.
//
// This file is part of Groom.
//
// Groom is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Groom is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Groom.  If not, see <http://www.gnu.org/licenses/>.

//! # Groom
//!
//! Groom is primarily a Command-Line Interface (CLI) application for processing
//! [mustache](https://mustache.github.io/) templates, but it is also implemented as a library
//! (crate) that can be integrated with other projects. Since the binary is essentially a wrapper
//! around the publicly exposed functionality of the crate, documentation on using the binary is
//! provided here in the API documentation.
//!
//! ## Binary Usage
//!
//! The following examples show using the binary (executable) from a command line. The binary
//! should work on Windows, macOS, and Linux.
//!
//! ### Examples
//!
//! The following example will render the input template using the data in the `data.yml` file and
//! writes the output to `stdout`.
//!
//! ```bash
//! $ groom -m data.yml template.mustache
//! ```
//!
//! The following example renders the input template by reading the data from `stdin` and writes
//! the output to `stdout`.
//!
//! ```bash
//! $ cat data.yml | groom template.mustache
//! ```
//!
//! The following example renders the input template using the data in the `data.yml` file and
//! writes the output to a file.
//!
//! ```bash
//! $ groom -m data.yml template.mustache output
//! ```
//!
//! ### Exit Codes
//!
//! | Code | Reason                               |
//! |------|--------------------------------------|
//! | 0    | Success, no error                    |
//! | 1    | Failure, generic                     |
//! | 2    | Failure, user input                  |
//! | 3    | Failure, Input/Output (IO)           |
//! | 4    | Failure, rendering mustache template |
//! | 5    | Failure, UTF8 encoding               |
//! | 6    | Failure, YAML decoding               |

#[macro_use]
extern crate log;
extern crate mustache;
extern crate serde_yaml;

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;
use std::str;

pub use self::groom::Groom;

mod groom;

/// A specialized `Result` type for groom operations.
pub type Result<T> = result::Result<T, Error>;

/// The error type for groom-related operations and associated traits.
///
/// Errors mostly originate from the dependencies, but custom instances of Error can be created
/// with the `Generic` variant and a message. If the error is related to user input, then the
/// `Input` variant should be used.
#[derive(Debug)]
pub enum Error {
    /// A generic or custom error occurred. The message should contain the detailed information.
    Generic(String),
    /// An error occurred with end-user input, such as a typo in a path.
    Input(String),
    /// An I/O operation failed.
    Io(io::Error),
    /// Parsing and rendering a mustache template failed.
    Mustache(mustache::Error),
    /// Converting to and/or from a UTF8 string failed.
    Utf8(str::Utf8Error),
    /// Decoding or deserializing YAML data failed.
    Yaml(serde_yaml::Error),
}

impl Error {
    /// Gets an error code related to the error.
    ///
    /// This is useful as a return, or exit, code for a command line application, where a non-zero
    /// integer indicates a failure in the application. It can also be used for quickly and easily
    /// testing equality between two errors.
    pub fn code(&self) -> i32 {
        match *self {
            Error::Generic(..) => 1,
            Error::Input(..) => 2,
            Error::Io(..) => 3,
            Error::Mustache(..) => 4,
            Error::Utf8(..) => 5,
            Error::Yaml(..) => 6,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Generic(ref msg) => write!(f, "{}", msg),
            Error::Input(ref msg) => write!(f, "{}", msg),
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Mustache(ref err) => write!(f, "{}", err),
            Error::Utf8(ref err) => write!(f, "{}", err),
            Error::Yaml(ref err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Generic(..) => "Generic",
            Error::Input(..) => "Input",
            Error::Io(..) => "IO",
            Error::Mustache(..) => "Mustache",
            Error::Utf8(..) => "UTF8",
            Error::Yaml(..) => "YAML",
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Mustache(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::Yaml(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<mustache::Error> for Error {
    fn from(err: mustache::Error) -> Error {
        Error::Mustache(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}
