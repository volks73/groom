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
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Read, Write};
use std::str;
use Result;

/// A builder for running the application.
pub struct Groom {
    input: Option<PathBuf>,
    output: Option<PathBuf>
}

impl Groom {
    /// Creates a new application instance.
    pub fn new() -> Groom {
        Groom {
            input: None,
            output: None,
        }
    }

    /// Sets the inputs.
    ///
    /// If the inputs is `None` or the vector is empty, then `stdin` is used for the input
    /// template.
    pub fn input(mut self, i: Option<&str>) -> Self {
        self.input = i.map(|i| PathBuf::from(i));
        self
    }

    /// Sets the output.
    ///
    /// If the output is `None`, then `stdout` is used for the output stream.
    pub fn output(mut self, o: Option<&str>) -> Self {
        self.output = o.map(|o| PathBuf::from(o));
        self
    }

    /// Runs the application.
    ///
    /// This will process the input template using the provided mapping and write to the output.
    pub fn run(self, m: &str) -> Result<()> {
        let mapping = PathBuf::from(m);
        debug!("mapping = {}", mapping.display());
        debug!("input = {:?}", self.input);
        debug!("output = {:?}", self.output);
        let value: Value = serde_yaml::from_reader(File::open(mapping)?)?;
        let template = if let Some(input) = self.input {
            trace!("Reading template from '{}'", input.display());
            mustache::compile_path(input)?
        } else {
            info!("Reading template from stdin");
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            mustache::compile_str(str::from_utf8(&buffer)?)?
        };
        let output_writer: Box<Write> = if let Some(output) = self.output {
            trace!("Writing rendering to '{}'", output.display());
            Box::new(File::create(output)?)
        } else {
            info!("Writing rendering to stdout");
            Box::new(io::stdout())
        };
        Ok(())
    }
}

