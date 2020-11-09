use crate::project::parse::Parse;
use crate::project::project_error::{ProjectError};
use crate::project::project_error;
use super::dependency::Dependency;

use snafu::{ResultExt};
use serde::{Serialize, Deserialize};
// The fetchfile (located in fetch.yml) describes
// where and how to download a dependency.

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticActual {
    pub files: Option<Vec<String>>,
    pub dependencies: Option<Vec<Dependency>>,
}

impl StaticActual {
    pub fn new() -> Self {
        Self {
            files: None,
            dependencies: None,
        }
    }
}

impl Parse for StaticActual {
    fn from_string(&mut self, src: &str) -> Result<(), ProjectError> {
        *self = serde_yaml::from_str(src).context(
            project_error::ParseFile {
                filetype: "fetchfile",
            }
        )?;

        Ok(())
    }
}

/* fn from_string(&mut self, src: &str) -> Result<(), ProjectError>; */
