use std::{
    fmt,
    error::Error,
};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KbtError;

impl Error for KbtError {
    fn description(&self) -> &str {
        "Kobuta error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for KbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kobuta error")
    }
}
