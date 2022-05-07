use winapi::um::gdiplustypes::REAL;

use crate::enums::Status;

#[derive(Debug)]
pub enum Error {
    Code(u32),
    String(String),
    Status(Status),
}
impl From<u32> for Error {
    fn from(val: u32) -> Error {
        match Status::try_from(val) {
            Ok(status) => Error::Status(status),
            Err(err) => err,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

/// The format is (x, y)
pub type Point = (REAL, REAL);
