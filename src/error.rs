#[derive(Debug)]
pub enum KvError {
    InvalidArgument,
    IOError,
}

pub type Result<T> = std::result::Result<T, KvError>;
