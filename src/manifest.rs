use std::collections::HashMap;

use serde::Deserialize;

pub type Manifest = HashMap<String, Entry>;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub file: String,
    #[serde(default)]
    pub css: Vec<String>,
}
