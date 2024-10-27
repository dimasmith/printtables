use std::fmt::Display;

use super::validation::ValidationError;

/// Name of the project.
/// Name cannot be empty and it's max lenght is 200 bytes.
#[derive(Debug, Clone)]
pub struct Name(String);

impl Name {
    pub fn parse(s: impl Into<String>) -> Result<Self, ValidationError> {
        let value = s.into();
        let trimmed = value.trim();
        let attr = "name";
        if trimmed.is_empty() {
            return Err(ValidationError::new(
                attr,
                "project.name.too-short",
                "project name is too short",
            ));
        }
        if trimmed.len() > 200 {
            return Err(ValidationError::new(
                attr,
                "project.name.too-long",
                "project name is too long",
            ));
        }
        Ok(Self(trimmed.to_string()))
    }
}

impl TryFrom<String> for Name {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&str> for Name {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn accept_correct_name() {
        let ok = Name::try_from("Multiboard Wall");
        assert!(ok.is_ok());
        assert_eq!(ok.unwrap().as_ref(), "Multiboard Wall");
    }

    #[test]
    fn accept_long_name() {
        let res = Name::try_from("a".repeat(200));
        assert!(res.is_ok());
    }

    #[test]
    fn reject_empty_name() {
        let err = Name::try_from("".to_string());
        assert!(err.is_err());
    }

    #[test]
    fn reject_blank_name() {
        let err = Name::try_from("    ".to_string());
        assert!(err.is_err());
    }

    #[test]
    fn reject_long_name() {
        let err = Name::try_from("a".repeat(201));
        assert!(err.is_err());
    }
}
