use parst::{error::Error, Deparsable, PResult, PResultBytes, Parsable};

pub fn try_split_at<S>(input: &[S], at: usize) -> Option<(&[S], &[S])> {
	(input.len() >= at).then(|| input.split_at(at))
}

#[derive(Debug, Clone, Default)]
pub struct VarBytes<'a, L> {
	length: L,
	slice: &'a [u8],
}

impl<L> AsRef<[u8]> for VarBytes<'_, L> {
	fn as_ref(&self) -> &[u8] { self.slice }
}

impl<'a, L> Parsable<'a, [u8]> for VarBytes<'a, L>
where
	L: Copy + Into<u64> + Parsable<'a, [u8], ()>,
{
	fn read(source: &'a [u8], _context: (), index: usize) -> PResultBytes<Self> {
		let (length, source, index) = L::read(source, (), index)?;
		let (slice, source) =
			try_split_at(source, length.into() as _).ok_or((Error::NotEnoughBytes, index))?;

		Ok((
			Self { length, slice },
			source,
			index + length.into() as usize,
		))
	}
}

impl<L> Deparsable for VarBytes<'_, L>
where
	L: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.length.write(&mut *w)?;
		self.slice.write(&mut *w)?;
		Ok(())
	}
}

#[derive(Debug, Clone, Default)]
pub struct VarStructs<L, T> {
	length: L,
	vec: Vec<T>,
}

impl<L, T> AsRef<[T]> for VarStructs<L, T> {
	fn as_ref(&self) -> &[T] { &self.vec }
}

impl<'a, S, Ctx, L, T> Parsable<'a, S, Ctx> for VarStructs<L, T>
where
	Ctx: Copy,
	L: Copy + Into<u64> + Parsable<'a, S, ()>,
	T: Parsable<'a, S, Ctx>,
{
	fn read(source: &'a S, context: Ctx, index: usize) -> PResult<Self, S> {
		let (length, mut source, mut index) = L::read(source, (), index)?;
		let vec = (0..length.into())
			.map(|_| {
				let (t, tail, index_new) = T::read(source, context, index)?;
				source = tail;
				index = index_new;
				Ok(t)
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok((Self { length, vec }, source, index))
	}
}

impl<L, T> Deparsable for VarStructs<L, T>
where
	L: Deparsable,
	T: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.length.write(&mut *w)?;
		self.vec.write(&mut *w)?;
		Ok(())
	}
}
