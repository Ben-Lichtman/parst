use crate::{error::Error, Deparsable, PResult, Parsable};
use std::borrow::Cow;

fn try_split_at(input: &[u8], at: usize) -> PResult<&[u8]> {
	(input.len() >= at)
		.then(|| input.split_at(at))
		.ok_or(Error::NotEnoughBytes)
}

#[derive(Clone, Debug, Default)]
pub struct Never;

impl<'a, C> Parsable<'a, C> for Never {
	fn read(_bytes: &'a [u8], _context: C) -> PResult<'a, Self> {
		panic!("Attempted to parse a never parsable struct")
	}
}

impl Deparsable for Never {
	fn write(&self, _w: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
}

#[derive(Clone, Debug, Default)]
pub struct VarBytes<'a, L> {
	pub length: L,
	pub slice: &'a [u8],
}

impl<'a, L> TryFrom<&'a [u8]> for VarBytes<'a, L>
where
	L: TryFrom<usize>,
{
	type Error = L::Error;

	fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
		let length = L::try_from(input.len())?;
		let slice = input;
		Ok(Self { length, slice })
	}
}

impl<L> AsRef<[u8]> for VarBytes<'_, L> {
	fn as_ref(&self) -> &[u8] { self.slice }
}

impl<'a, C, L> Parsable<'a, C> for VarBytes<'a, L>
where
	L: Copy + Into<u64> + Parsable<'a, ()>,
{
	fn read(bytes: &'a [u8], _context: C) -> PResult<Self> {
		let (length, bytes) = L::read(bytes, ())?;
		let (slice, bytes) = try_split_at(bytes, length.into() as _)?;
		Ok((Self { length, slice }, bytes))
	}
}

impl<L> Deparsable for VarBytes<'_, L>
where
	L: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.length.write(w)?;
		self.slice.write(w)?;
		Ok(())
	}
}

#[derive(Clone, Debug, Default)]
pub struct VarBytesCow<'a, L> {
	pub length: L,
	pub cow: Cow<'a, [u8]>,
}

impl<'a, L> TryFrom<&'a [u8]> for VarBytesCow<'a, L>
where
	L: TryFrom<usize>,
{
	type Error = L::Error;

	fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
		let length = L::try_from(input.len())?;
		let cow = Cow::from(input);
		Ok(Self { length, cow })
	}
}

impl<L> TryFrom<Vec<u8>> for VarBytesCow<'_, L>
where
	L: TryFrom<usize>,
{
	type Error = L::Error;

	fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
		let length = L::try_from(input.len())?;
		let cow = Cow::from(input);
		Ok(Self { length, cow })
	}
}

impl<L> AsRef<[u8]> for VarBytesCow<'_, L> {
	fn as_ref(&self) -> &[u8] { &self.cow }
}

impl<L> AsMut<Vec<u8>> for VarBytesCow<'_, L> {
	fn as_mut(&mut self) -> &mut Vec<u8> { self.cow.to_mut() }
}

impl<'a, C, L> Parsable<'a, C> for VarBytesCow<'a, L>
where
	L: Copy + Into<u64> + Parsable<'a, ()>,
{
	fn read(bytes: &'a [u8], _context: C) -> PResult<Self> {
		let (length, bytes) = L::read(bytes, ())?;
		let (slice, bytes) = try_split_at(bytes, length.into() as _)?;
		let cow = Cow::from(slice);
		Ok((Self { length, cow }, bytes))
	}
}

impl<L> Deparsable for VarBytesCow<'_, L>
where
	L: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.length.write(w)?;
		self.cow.write(w)?;
		Ok(())
	}
}

#[derive(Clone, Debug, Default)]
pub struct VarBytesOwned<L> {
	pub length: L,
	pub vec: Vec<u8>,
}

impl<L> TryFrom<Vec<u8>> for VarBytesOwned<L>
where
	L: TryFrom<usize>,
{
	type Error = L::Error;

	fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
		let length = L::try_from(input.len())?;
		let vec = input;
		Ok(Self { length, vec })
	}
}

impl<L> AsRef<[u8]> for VarBytesOwned<L> {
	fn as_ref(&self) -> &[u8] { &self.vec }
}

impl<L> AsMut<Vec<u8>> for VarBytesOwned<L> {
	fn as_mut(&mut self) -> &mut Vec<u8> { &mut self.vec }
}

impl<'a, C, L> Parsable<'a, C> for VarBytesOwned<L>
where
	L: Copy + Into<u64> + Parsable<'a, ()>,
{
	fn read(bytes: &'a [u8], _context: C) -> PResult<Self> {
		let (length, bytes) = L::read(bytes, ())?;
		let (slice, bytes) = try_split_at(bytes, length.into() as _)?;
		let vec = Vec::from(slice);
		Ok((Self { length, vec }, bytes))
	}
}

impl<L> Deparsable for VarBytesOwned<L>
where
	L: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.length.write(w)?;
		self.vec.as_slice().write(w)?;
		Ok(())
	}
}

#[derive(Clone, Default, Debug)]
pub struct VarStructs<L, T> {
	pub length: L,
	pub vec: Vec<T>,
}

impl<L, T> AsRef<[T]> for VarStructs<L, T> {
	fn as_ref(&self) -> &[T] { &self.vec }
}

impl<L, T> AsMut<[T]> for VarStructs<L, T> {
	fn as_mut(&mut self) -> &mut [T] { &mut self.vec }
}

impl<'a, C, L, T> Parsable<'a, C> for VarStructs<L, T>
where
	C: Clone,
	L: Copy + Into<u64> + Parsable<'a, ()>,
	T: Parsable<'a, C>,
{
	fn read(bytes: &'a [u8], context: C) -> PResult<Self> {
		let (length, mut bytes) = L::read(bytes, ())?;
		let vec = (0..length.into())
			.map(|_| {
				let (t, tail) = T::read(bytes, context.clone())?;
				bytes = tail;
				Ok(t)
			})
			.collect::<Result<Vec<_>, _>>()?;
		Ok((Self { length, vec }, bytes))
	}
}

impl<L, T> Deparsable for VarStructs<L, T>
where
	L: Deparsable,
	T: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.length.write(w)?;
		self.vec.write(w)?;
		Ok(())
	}
}
