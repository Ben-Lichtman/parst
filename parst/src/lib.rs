#![feature(array_from_fn)]

pub mod error;
#[cfg(feature = "extra")]
pub mod extra;

mod primitives;

use std::io::Write;

#[cfg(feature = "derive")]
pub use parst_derive::Parsable;

use crate::error::Error;

pub type PResult<'a, O, E = Error> = std::result::Result<(O, &'a [u8]), E>;

pub trait Parsable<'a, C>: Sized
where
	C: Copy,
{
	fn read(bytes: &'a [u8], context: C) -> PResult<'a, Self>;
}

pub trait Deparsable {
	fn write(&self, w: impl Write) -> std::io::Result<()>;
}
