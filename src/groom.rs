use std::path::PathBuf;
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
    /// This will process the input templates using the provided mapping.
    pub fn run(self, m: &str) -> Result<()> {
        let mapping = PathBuf::from(m);
        debug!("mapping = {}", mapping.display());
        debug!("input = {:?}", self.input);
        debug!("output = {:?}", self.output);
        // TODO: Add converting inputs to stdin if None
        // TODO: Add converting output to stdout if None
        // TODO: Add processing templates and writing to output
        Ok(())
    }
}

