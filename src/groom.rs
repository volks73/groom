use std::path::PathBuf;
use std::ffi::OsStr;
use Result;

/// A builder for running the application.
pub struct Groom {
    inputs: Option<Vec<PathBuf>>,
    output: Option<PathBuf>
}

impl Groom {
    pub fn new() -> Groom {
        Groom {
            inputs: None,
            output: None,
        }
    }

    pub fn input<S: AsRef<OsStr>>(mut self, i: Option<Vec<S>>) -> Self {
        self.inputs = i.map(|v| v.iter().map(|s| PathBuf::from(s)).collect());
        self
    }

    pub fn output(mut self, o: Option<&str>) -> Self {
        self.output = o.map(|o| PathBuf::from(o));
        self
    }

    pub fn run(self, mapping: &str) -> Result<()> {
        debug!("mapping = {}", mapping);
        Ok(())
    }
}

