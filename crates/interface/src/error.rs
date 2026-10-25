use core::fmt;

pub trait Error: core::fmt::Debug {
    fn kind(&self) -> ErrorKind;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotConnected,
    Other,
}

pub trait ErrorType {
    type Error: Error;
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotConnected => f.write_str("not connected to the host"),
            Self::Other => f.write_str("other error occurred"),
        }
    }
}
