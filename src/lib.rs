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
//! [mustache](https://mustache.github.io/) templates.

#[macro_use] extern crate log;
extern crate serde_yaml;

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
    Io(io::Error),
    Yaml(serde_yaml::Error),   
}

impl Error {
    pub fn code(&self) -> i32 {
        match *self{
            Error::Generic(..) => 1,
            Error::Input(..) => 2,
            Error::Io(..) => 3,
            Error::Yaml(..) => 4,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Generic(ref msg) => write!(f, "{}", msg),
            Error::Input(ref msg) => write!(f, "{}", msg),
            Error::Io(ref err) => write!(f, "{}", err),
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
            Error::Yaml(..) => "YAML",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Yaml(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}

