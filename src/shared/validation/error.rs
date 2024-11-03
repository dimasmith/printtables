//! Validation error.

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error to report invalid value of attributes.
///
/// The `attribute` keeps and attribute name, e.g., the name of the invalid struct field.
/// The `code` is for unique machine-readable error code. `project.name.empty` is one example.
/// Follow the convention `<object>.<attribute>.<error-type>` where applicable.
/// The UI uses error codes to internationalize error messages.
/// The `message` is human-readable error message. It is displayed when there is no
/// internationalized variant available.
#[derive(Debug, Error, Serialize, Deserialize)]
pub struct ValidationError {
    attribute: String,
    code: String,
    message: String,
}

impl ValidationError {
    pub fn new(
        attribute: impl Into<String>,
        error_code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            attribute: attribute.into(),
            code: error_code.into(),
            message: message.into(),
        }
    }

    pub fn attribute(&self) -> &str {
        &self.attribute
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid {} value: {}", self.attribute, self.message)
    }
}
