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

use mustache;
use serde_yaml::{self, Value};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use Error;
use Result;

/// A builder for running the application.
pub struct Groom {
    data: Option<PathBuf>,
    output: Option<PathBuf>,
}

impl Groom {
    /// Creates a new application instance.
    pub fn new() -> Groom {
        Groom {
            data: None,
            output: None,
        }
    }

    /// Sets the data.
    ///
    /// If the data is `None`, then `stdin` is used for the data.
    pub fn data<P: AsRef<Path>>(mut self, d: Option<P>) -> Self {
        self.data = d.map(|d| PathBuf::from(d.as_ref()));
        self
    }

    /// Sets the output.
    ///
    /// If the output is `None`, then `stdout` is used for the output stream.
    pub fn output<P: AsRef<Path>>(mut self, o: Option<P>) -> Self {
        self.output = o.map(|o| PathBuf::from(o.as_ref()));
        self
    }

    /// Runs the application.
    ///
    /// This will consume the `Groom` and process the input template using the provided mapping and
    /// write to the output.
    pub fn run<P: AsRef<Path>>(self, input: P) -> Result<()> {
        debug!("data = {:?}", self.data);
        debug!("output = {:?}", self.output);
        let data: Value = if let Some(data) = self.data {
            trace!("Reading data from '{}'", data.display());
            if data.exists() {
                serde_yaml::from_reader(File::open(data)?)?
            } else {
                return Err(Error::Input(format!(
                    "The '{}' data file does not exist",
                    data.display()
                )));
            }
        } else {
            info!("Reading data from stdin");
            serde_yaml::from_reader(io::stdin())?
        };
        let mut output_writer: Box<dyn Write> = if let Some(output) = self.output {
            trace!("Rendering to '{}'", output.display());
            Box::new(File::create(output)?)
        } else {
            info!("Rendering to stdout");
            Box::new(io::stdout())
        };
        let path = PathBuf::from(input.as_ref());
        if path.exists() {
            info!("Compiling '{}'", path.display());
            let template = mustache::compile_path(&path)?;
            info!("Rendering '{}'", path.display());
            // A pull request has been sent to the upstream project to add serde support. Until it is
            // accepted/merged, the https://github.com/volks73/rust-mustache.git repository is used,
            // which does contain serde support and development can continue.
            template.render(&mut output_writer, &data)?;
        } else {
            return Err(Error::Input(format!(
                "The '{}' template file does not exist",
                path.display()
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_correct() {
        let groom = Groom::new();
        assert!(groom.data.is_none());
        assert!(groom.output.is_none());
    }

    #[test]
    fn output_works() {
        const EXPECTED: &str = "test";
        let groom = Groom::new().output(Some(EXPECTED));
        assert_eq!(groom.output, Some(PathBuf::from(EXPECTED)));
    }

    #[test]
    fn data_works() {
        const EXPECTED: &str = "test";
        let groom = Groom::new().data(Some(EXPECTED));
        assert_eq!(groom.data, Some(PathBuf::from(EXPECTED)));
    }
}
