use std::io;

#[derive(Debug)]
pub enum KvError {
    InvalidArgument,
    IOError,
}

pub type Result<T> = std::result::Result<T, KvError>;

impl From<io::Error> for KvError {
    fn from(_: io::Error) -> Self {
        KvError::IOError
    }
}

