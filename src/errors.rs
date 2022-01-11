use bincode::ErrorKind as BincodeErrorKind;
use serde_json::error::Category as SerdeCategory;
use serde_json::Error as SerdeError;
use std::io::Error as StdioError;
use std::io::ErrorKind as StdioErrorKind;

pub enum DatabaseError {
    BinCode(BincodeErrorKind),
    StdioError(StdioErrorKind),
    SerdeError(SerdeCategory),
}

impl From<Box<BincodeErrorKind>> for DatabaseError {
    fn from(boxed_err: Box<BincodeErrorKind>) -> Self {
        Self::BinCode(*boxed_err.as_ref())
    }
}

impl From<SerdeError> for DatabaseError {
    fn from(serde_err: SerdeError) -> Self {
        Self::SerdeError(serde_err.classify())
    }
}

impl From<StdioError> for DatabaseError {
    fn from(stdio_err: StdioError) -> Self {
        Self::StdioError(stdio_err.kind())
    }
}
