//! Module with functionality for using & loading data
//! from the `request.json`

use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::io::Read;

/// Type for interfacing with data from `request.json
#[derive(Serialize, Deserialize)]
pub struct IncomingRequest {
    name: String
}

impl IncomingRequest {
    /// Load the `request.json` from the filesystem
    pub fn load() -> Result<Self> {
        let mut fp = std::fs::File::options()
            .read(true)
            .open("request.json")?;

        // file_contents is the plain-text file contents of `request.json`
        let mut file_contents = String::new();
        fp.read_to_string(&mut file_contents)?;

        Ok(serde_json::from_str::<Self>(&file_contents)?)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

