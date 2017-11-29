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

use Error;
use mustache;
use serde_yaml::{self, Value};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Write};
use Result;

// TODO: Add `data-format` method. This would indicate the format of the data, i.e. JSON, YAML,
// Msgpack, Bincode, etc. This would override the format determined from the file extension.
// THe default would be YAML.

// TODO: Add a `serve` method. This would start process and render the templates but then start
// a basic http server and serve the rendered templates. The `serve` method would take an
// optional port number. If `None`, then port 8080 would be used. Pages would be served at
// http://localhost:8080. If no output file is specified, then the file name of the template is
// used but the file extension is replaced with `.html`.

// TODO: Add a `listen` method. This would be after adding the `serve` method. It would
// establish a file system notification for any modification of the template file. If the file
// is modified, groom would automatically re-render the template; thus, the web page served
// using the `serve` method would automatically update on modification.

// TODO: Add data format deserialization based on file extension. For example, if the data file is
// data.json, then the data is read as JSON instead of YAML. Or, if the data file is data.msgpack,
// then the data is read as MessagePack, but it all gets used the same way for rendering. This
// would allow different data formats to be used for rendering templates.

// TODO: Add support for handlebars in additon to mustache templates. The file extension would
// determine the format and which crate is used.

// TODO: Add `template-format` method. This would override the file extension detection system and
// explicitly indicate the template engine to use: (1) mustache or (2) handlebars.

/// A builder for running the application.
pub struct Groom {
    data: Option<PathBuf>,
    output: Option<PathBuf>
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
                return Err(Error::Input(format!("The '{}' data file does not exist", data.display())))
            }
        } else {
            info!("Reading data from stdin");
            serde_yaml::from_reader(io::stdin())?
        };
        let mut output_writer: Box<Write> = if let Some(output) = self.output {
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
            return Err(Error::Input(format!("The '{}' template file does not exist", path.display())));
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

