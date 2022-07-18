//! Scary logic /s
//! Really just formatting a string to greet the caller
use std::io::Write;
use crate::req;
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// Type for generating response
#[derive(Serialize, Deserialize)]
pub struct Response {
    greeting: String,
}

/// Make it so we can generate a response from a greeting
impl From<req::IncomingRequest> for Response {
    fn from(request: req::IncomingRequest) -> Self {
        let greeting = format!("Hello, it's nice to meet you, {}", request.name());
        Self {
            greeting
        }
    }
}

impl Response {
    /// Save the response.
    pub fn save(&self) -> Result<()> {
        let mut fp = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open("response.json")?;
        let out_file_contents = serde_json::to_string(&self)?;
        write!(fp, "{}", out_file_contents)?;
        Ok(())
    }
}
