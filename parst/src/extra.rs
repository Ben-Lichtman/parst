use crate::{error::Error, Deparsable, PResult, Parsable};

fn try_split_at(input: &[u8], at: usize) -> PResult<&[u8]> {
	(input.len() >= at)
		.then(|| input.split_at(at))
		.ok_or(Error::NotEnoughBytes)
}

#[derive(Debug, Default)]
pub struct VarBytes<'a, L> {
	length: L,
	slice: &'a [u8],
}

impl<L> AsRef<[u8]> for VarBytes<'_, L> {
	fn as_ref(&self) -> &[u8] { self.slice }
}

impl<'a, C, L> Parsable<'a, C> for VarBytes<'a, L>
where
	C: Copy,
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
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.length.write(&mut w)?;
		self.slice.write(&mut w)?;
		Ok(())
	}
}

#[derive(Debug, Default)]
pub struct VarStructs<L, T> {
	length: L,
	vec: Vec<T>,
}

impl<L, T> AsRef<[T]> for VarStructs<L, T> {
	fn as_ref(&self) -> &[T] { &self.vec }
}

impl<'a, C, L, T> Parsable<'a, C> for VarStructs<L, T>
where
	C: Copy,
	L: Copy + Into<u64> + Parsable<'a, ()>,
	T: Parsable<'a, C>,
{
	fn read(bytes: &'a [u8], context: C) -> PResult<Self> {
		let (length, mut bytes) = L::read(bytes, ())?;
		let vec = (0..length.into())
			.map(|_| {
				let (t, tail) = T::read(bytes, context)?;
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
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.length.write(&mut w)?;
		self.vec.write(&mut w)?;
		Ok(())
	}
}
