pub mod parse;

use serde::{Serialize, Deserialize};

/// Metadata (located in hy.yml) describes
/// certain data about the project (like names and authors).
/// It can contain static actuals, but no
/// dependency fetching rules.
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    name: String,
    // kind: Type,
    description: String,
    authors: Vec<String>,
    version: String,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            name: "".into(),
            description: "".into(),
            authors: vec![],
            version: "".into(),
        }
    }
}
