use std::fmt;

// Implement a custom error type:
// https://learning-rust.github.io/docs/e7.custom_error_types.html
#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot create pool")
    }
}
