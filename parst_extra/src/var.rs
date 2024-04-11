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
	fn read(source: &'a [u8], _context: ()) -> PResultBytes<Self> {
		let (length, source) = L::read(source, ())?;
		let (slice, source) =
			try_split_at(source, length.into() as _).ok_or((Error::NotEnoughBytes, source))?;

		Ok((Self { length, slice }, source))
	}
}

impl<L> Deparsable for VarBytes<'_, L>
where
	L: Deparsable,
{
	fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
		self.length.write(&mut *w, ())?;
		self.slice.write(&mut *w, ())?;
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
	S: ?Sized,
	Ctx: Copy,
	L: Copy + Into<u64> + Parsable<'a, S, ()>,
	T: Parsable<'a, S, Ctx>,
{
	fn read(source: &'a S, context: Ctx) -> PResult<Self, S> {
		let (length, mut source) = L::read(source, ())?;
		let vec = (0..length.into())
			.map(|_| {
				let (t, tail) = T::read(source, context)?;
				source = tail;
				Ok(t)
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok((Self { length, vec }, source))
	}
}

impl<L, T> Deparsable for VarStructs<L, T>
where
	L: Deparsable,
	T: Deparsable,
{
	fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
		self.length.write(&mut *w, ())?;
		self.vec.write(&mut *w, ())?;
		Ok(())
	}
}

pub struct ConsumingVec<T>(Vec<T>);

impl<'a, Ctx, T> Parsable<'a, [u8], Ctx> for ConsumingVec<T>
where
	Ctx: Copy,
	T: Parsable<'a, [u8], Ctx>,
{
	#[inline]
	fn read(mut source: &'a [u8], context: Ctx) -> PResult<Self, [u8]> {
		let mut v = Vec::new();
		while let Ok((element, remainder)) = Parsable::read(source, context) {
			v.push(element);
			source = remainder;
		}
		if !source.is_empty() {
			return Err((Error::InvalidInput, source));
		}
		Ok((ConsumingVec(v), source))
	}
}

impl<T, Ctx> Deparsable<Ctx> for ConsumingVec<T>
where
	Ctx: Copy,
	T: Deparsable<Ctx>,
{
	#[inline]
	fn write(&mut self, w: &mut impl std::io::Write, context: Ctx) -> std::io::Result<()> {
		for element in self.0.as_mut_slice() {
			element.write(&mut *w, context)?;
		}
		Ok(())
	}
}

impl<T> AsRef<[T]> for ConsumingVec<T> {
	fn as_ref(&self) -> &[T] { self.0.as_ref() }
}
