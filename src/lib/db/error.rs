use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ZeroConnection;

impl Display for ZeroConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pool connections can't be zero")
    }
}

impl Error for ZeroConnection {}
