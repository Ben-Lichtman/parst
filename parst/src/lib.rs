#![feature(split_array)]
#![feature(array_try_from_fn)]

#[cfg(feature = "endian")]
pub mod endian;
pub mod error;

pub(crate) mod helpers;

mod collections;
mod primitives;

use std::io::Write;

#[cfg(feature = "derive")]
pub use parst_derive::{Deparsable, Parsable};

pub type PResult<'a, O, S, E = crate::error::Error> =
	std::result::Result<(O, &'a S, usize), (E, usize)>;
pub type PResultBytes<'a, O> = PResult<'a, O, [u8]>;
pub type PResultStr<'a, O> = PResult<'a, O, str>;

pub type PResultUncounted<'a, O, S, E = crate::error::Error> = std::result::Result<(O, &'a S), E>;
pub type PResultBytesUncounted<'a, O> = PResult<'a, O, [u8]>;
pub type PResultStrUncounted<'a, O> = PResult<'a, O, str>;

pub trait Parsable<'a, Src, Ctx = ()>: Sized
where
	Src: ?Sized,
{
	fn read(source: &'a Src, context: Ctx, index: usize) -> PResult<Self, Src>;

	fn read_uncounted(source: &'a Src, context: Ctx) -> PResultUncounted<Self, Src> {
		match Self::read(source, context, 0) {
			Ok((value, src, _)) => Ok((value, src)),
			Err((e, _)) => Err(e),
		}
	}
}

pub trait Deparsable {
	fn write(&self, w: &mut impl Write) -> std::io::Result<()>;
}
