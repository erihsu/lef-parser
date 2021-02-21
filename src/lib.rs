mod model;
mod parser;

pub use model::LefData;
use nom::{
    error::{convert_error, VerboseError},
    Err, IResult,
};
use parser::lef_parser::*;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

impl FromStr for LefData {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match lef_parser(s) {
            Ok((_, u)) => Ok(u),
            Err(Err::Error(e)) => {
                println!("[LEFParser] `VerboseError`:\n{}", convert_error(s, e));
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid Technology File",
                ))
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid Technology File",
            )),
        }
    }
}

pub type LefRes<T, U> = IResult<T, U, VerboseError<T>>;
