#![feature(array_from_fn)]

pub mod error;

mod primitives;

use crate::error::{Error, Result};
use std::io::Write;

pub type PResult<'a, O, E = Error> = std::result::Result<(O, &'a [u8]), E>;

pub trait Parsable<'a>: Sized {
	fn read(bytes: &'a [u8]) -> PResult<Self>;
}

pub trait Serializeable<'a> {
	fn write(w: impl Write) -> Result<()>;
}
