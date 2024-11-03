//! Gathers multiple validation errors into a single list.

use super::error::ValidationError;

#[derive(Debug, Default)]
pub struct CollectingValidator {
    errors: Vec<ValidationError>,
}

impl CollectingValidator {
    /// Parses payload into value object.
    /// Returns parsing result.
    /// If there were an error during parsing, the validator stores the error internally.
    /// After all values are parsed, the user must check if errors happened using has_errors
    /// method.
    /// If no errors were reported, it is safe to build domain object by calling unwrap on parsed
    /// values.
    pub fn parse<V, P>(&mut self, payload: P) -> Result<V, ValidationError>
    where
        V: TryFrom<P, Error = ValidationError>,
    {
        let resutl = V::try_from(payload);
        match resutl {
            Ok(value) => Ok(value),
            Err(invalid_payload) => {
                self.errors.push(invalid_payload.clone());
                Err(invalid_payload)
            }
        }
    }

    /// Convenience shortcut method to parse strings.
    /// As most of the payloads are string-based, it makes generics declaration on parse less
    /// verbose.
    pub fn parse_string<V>(&mut self, payload: impl Into<String>) -> Result<V, ValidationError>
    where
        V: TryFrom<String, Error = ValidationError>,
    {
        let payload = payload.into();
        self.parse(payload)
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn into_errors(self) -> Vec<ValidationError> {
        self.errors
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_valid_items() {
        let mut validator = CollectingValidator::default();
        let val_1 = validator.parse::<TestValue1, String>(VALID_VALUE.to_string());
        let val_2 = validator.parse::<TestValue2, String>(VALID_VALUE.to_string());

        assert!(
            !validator.has_errors(),
            "all values are valid. errors must not be reported"
        );

        assert!(val_1.is_ok());
        assert!(val_2.is_ok());

        let errors = validator.into_errors();
        has_no_errors(&errors, VALUE_1);
        has_no_errors(&errors, VALUE_2);
        assert!(errors.is_empty(), "no other errors must be reported");
    }

    #[test]
    fn parse_valid_and_invalid_items() {
        let mut validator = CollectingValidator::default();
        let val_1 = validator.parse::<TestValue1, String>(INVALID_VALUE.to_string());
        let val_2 = validator.parse::<TestValue2, String>(VALID_VALUE.to_string());

        assert!(
            validator.has_errors(),
            "errors must be reported when there is at least a single validation error"
        );

        assert!(val_1.is_err());
        assert!(val_2.is_ok());

        let errors = validator.into_errors();
        has_errors(&errors, VALUE_1);
        has_no_errors(&errors, VALUE_2);
        assert_eq!(
            errors.len(),
            1,
            "only one field has error. no other errors must be reported"
        );
    }
    #[test]
    fn parse_invalid_items() {
        let mut validator = CollectingValidator::default();
        let val_1 = validator.parse::<TestValue1, String>(INVALID_VALUE.to_string());
        let val_2 = validator.parse::<TestValue2, String>(INVALID_VALUE.to_string());

        assert!(
            validator.has_errors(),
            "errors must be reported. all valitations failed"
        );

        assert!(val_1.is_err());
        assert!(val_2.is_err());

        let errors = validator.into_errors();
        has_errors(&errors, VALUE_1);
        has_errors(&errors, VALUE_2);
        assert_eq!(
            errors.len(),
            2,
            "two fields are invalid. no other errors must be reported"
        );
    }

    fn has_errors(errors: &[ValidationError], value: &str) {
        let error_count = errors.iter().filter(|e| e.attribute() == value).count();
        assert!(
            error_count > 0,
            "value {} should have reported errors but there are none",
            value
        );
    }

    fn has_no_errors(errors: &[ValidationError], value: &str) {
        let error_count = errors.iter().filter(|e| e.attribute() == value).count();
        assert!(
            error_count == 0,
            "value {} should be valid but {} errors are reported",
            value,
            error_count
        );
    }

    const VALID_VALUE: &'static str = "valid_value";
    const INVALID_VALUE: &'static str = "invalid_value";
    const VALUE_1: &'static str = "value_1";
    const VALUE_2: &'static str = "value_2";

    struct TestValue1;
    struct TestValue2;

    impl TryFrom<String> for TestValue1 {
        type Error = ValidationError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.eq(VALID_VALUE) {
                Ok(Self)
            } else {
                Err(ValidationError::new(
                    VALUE_1,
                    "value_1.invalid",
                    "Invalid value 1",
                ))
            }
        }
    }

    impl TryFrom<String> for TestValue2 {
        type Error = ValidationError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.eq(VALID_VALUE) {
                Ok(Self)
            } else {
                Err(ValidationError::new(
                    VALUE_2,
                    "value_2.invalid",
                    "Invalid value 2",
                ))
            }
        }
    }
}
