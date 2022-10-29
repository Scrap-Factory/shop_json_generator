use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct GenDbError;

impl Display for GenDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to generate database")
    }
}

impl Error for GenDbError {}

#[derive(Debug)]
pub struct FromFileParseError;

impl Display for FromFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to parse from file")
    }
}

impl Error for FromFileParseError {}

#[derive(Debug)]
pub struct GenSetError;

impl Display for GenSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to generate set")
    }
}

impl Error for GenSetError {}
