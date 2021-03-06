#![feature(array_try_from_fn)]

pub mod error;

#[cfg(feature = "endian")]
pub mod endian;
#[cfg(feature = "extra")]
pub mod extra;

mod primitives;

use std::io::Write;

#[cfg(feature = "derive")]
pub use parst_derive::{Deparsable, Parsable};

use crate::error::Error;

pub type PResult<'a, O, E = Error> = std::result::Result<(O, &'a [u8]), E>;

pub trait Parsable<'a, C>: Sized {
	fn read(bytes: &'a [u8], context: C) -> PResult<'a, Self>;
}

pub trait Deparsable {
	fn write(&self, w: &mut impl Write) -> std::io::Result<()>;
}
