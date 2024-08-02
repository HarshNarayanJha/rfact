use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fact {
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String,
}

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
