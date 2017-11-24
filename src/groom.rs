use Result;
use std::path::PathBuf;

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

    pub fn output(mut self, o: Option<&str>) -> Self {
        self.output = o.map(|o| PathBuf::from(o));
        self
    }

    pub fn run(self) -> Result<()> {
        Ok(())
    }
}

