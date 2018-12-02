#![feature(try_trait)]

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
    NoneE(std::option::NoneError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error { Error::Io(e) }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error { Error::Num(e) }
}

impl From<std::option::NoneError> for Error {
    fn from(e: std::option::NoneError) -> Error { Error::NoneE(e) }
}