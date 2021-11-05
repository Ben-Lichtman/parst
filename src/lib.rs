pub mod error;
pub mod primitives;

use std::io::Write;

use crate::error::{Error, Result};

pub type PResult<'a, O, E = Error> = std::result::Result<(O, &'a [u8]), E>;

pub trait Parsable<'a>: Sized {
	fn read(i: &'a [u8]) -> PResult<Self>;
}

pub trait Serializeable<'a> {
	fn write(w: impl Write) -> Result<()>;
}
