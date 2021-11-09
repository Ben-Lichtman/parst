#![feature(array_from_fn)]

pub mod error;

mod primitives;

#[cfg(feature = "derive")]
pub use parst_derive::Parsable;

use crate::error::Error;
use std::io::{self, Write};

pub type PResult<'a, O, E = Error> = std::result::Result<(O, &'a [u8]), E>;

pub trait Parsable<'a>: Sized {
	fn read(bytes: &'a [u8]) -> PResult<Self>;
}

pub trait Serializeable<'a> {
	fn write(&self, w: impl Write) -> io::Result<()>;
}
